#!/usr/bin/env python3
"""Test inventory behavior without installing tools or querying the network.
Run this file with Python 3.11 or newer after modifying the checker.
"""

import importlib.util
import json
from pathlib import Path
from types import SimpleNamespace
import tempfile
import unittest
from unittest.mock import patch

# Load the adjacent checker without running its command entrypoint.
SPEC = importlib.util.spec_from_file_location(
    "checker", Path(__file__).with_name("check_rust_environment.py"))
CHECKER = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(CHECKER)


class InventoryTests(unittest.TestCase):
    """Check missing, stale, and uncertain inventory outcomes.
    Mock external services and test data boundaries rather than fixed prose.
    """

    def test_stable_release_selection_excludes_yanked_and_prerelease(self):
        """Reject releases that cannot be offered as stable updates.
        Select by numeric version order instead of lexicographic string order.
        """
        entries = [{"vers": "1.9.0"}, {"vers": "1.10.0"},
                   {"vers": "9.0.0", "yanked": True}, {"vers": "10.0.0-beta.1"}]
        data = "\n".join(json.dumps(item) for item in entries)
        with patch.object(CHECKER, "fetch_text", return_value=data):
            self.assertEqual(CHECKER.latest_crate("rayon", 1), "1.10.0")
        self.assertEqual(CHECKER.release_status("1.0.0-beta.1", "1.0.0", "OUTDATED"), "UNKNOWN")
        self.assertEqual(CHECKER.release_status("1.9.0", "1.10.0", "OUTDATED"), "OUTDATED")

    def test_missing_and_broken_commands_remain_distinct(self):
        """Separate absent commands from installed tools that cannot run.
        Keep network failure from changing an observed local tool state.
        """
        args = SimpleNamespace(timeout=1, online=True)
        with patch.object(CHECKER, "run_command", return_value=(1, "", "no such command: audit")), \
                patch.object(CHECKER.shutil, "which", return_value="/cargo"), \
                patch.object(CHECKER, "latest_crate", side_effect=OSError("offline")):
            row = CHECKER.tool_row("audit", ["cargo", "audit", "--version"], Path.cwd(),
                                   args, package="cargo-audit")
        self.assertEqual(row["status"], "MISSING")
        self.assertEqual(row["latest_status"], "UNKNOWN")
        with patch.object(CHECKER, "run_command", return_value=(1, "", "cannot load shared library")), \
                patch.object(CHECKER.shutil, "which", return_value="/hyperfine"), \
                patch.object(CHECKER, "latest_crate", return_value="2.0.0"):
            row = CHECKER.tool_row("hyperfine", ["hyperfine", "--version"], Path.cwd(),
                                   args, package="hyperfine")
        self.assertEqual(row["status"], "BROKEN")

    def test_workspace_alias_optional_and_multiple_lock_versions(self):
        """Preserve declarations and all locked versions in a workspace.
        Avoid proposing a crates.io update for a git-sourced dependency.
        """
        with tempfile.TemporaryDirectory() as folder:
            root = Path(folder)
            manifest = root / "Cargo.toml"
            manifest.write_text('[workspace]\nmembers = []\n')
            (root / "Cargo.lock").write_text(
                'version = 3\n[[package]]\nname = "rayon"\nversion = "1.9.0"\n'
                'source = "registry+https://github.com/rust-lang/crates.io-index"\n'
                '[[package]]\nname = "rayon"\nversion = "1.10.0"\n'
                'source = "registry+https://github.com/rust-lang/crates.io-index"\n'
                '[[package]]\nname = "dashmap"\nversion = "1.0.0"\nsource = "git+https://example.org/map"\n')
            metadata = {"workspace_root": str(root), "workspace_members": ["member-id"], "packages": [{
                "id": "member-id", "name": "member", "rust_version": "1.80", "edition": "2021",
                "dependencies": [{"name": "rayon", "rename": "parallel", "req": "^1.9",
                                  "optional": True, "features": [], "source": "registry+index"}]}]}
            args = SimpleNamespace(timeout=1, online=True)
            before = (root / "Cargo.lock").read_bytes()
            with patch.object(CHECKER, "run_command", return_value=(0, json.dumps(metadata), "")) as run, \
                    patch.object(CHECKER, "latest_crate", return_value="1.11.0"):
                rows = CHECKER.project_rows(manifest, root, args)
            rayon = next(row for row in rows if row["name"] == "rayon")
            dashmap = next(row for row in rows if row["name"] == "dashmap")
            self.assertEqual(len(rayon["locked"]), 2)
            self.assertTrue(rayon["declarations"][0]["optional"])
            self.assertEqual(rayon["latest_status"], "NEWER_AVAILABLE")
            self.assertNotIn("offer", dashmap)
            self.assertIn("--locked", run.call_args.args[0])
            self.assertIn("--offline", run.call_args.args[0])
            self.assertEqual(before, (root / "Cargo.lock").read_bytes())

    def test_failed_metadata_does_not_claim_libraries_absent(self):
        """Keep unverified dependency absence unknown on Cargo failure.
        Do not create a lockfile or resolve dependencies as a fallback.
        """
        with tempfile.TemporaryDirectory() as folder:
            root = Path(folder)
            manifest = root / "Cargo.toml"
            manifest.write_text('[workspace]\nmembers = []\n')
            with patch.object(CHECKER, "run_command", return_value=(1, "", "toolchain unavailable")):
                rows = CHECKER.project_rows(manifest, root, SimpleNamespace(timeout=1, online=False))
            self.assertTrue(all(row["status"] == "UNKNOWN" for row in rows if row["kind"] == "library"))
            self.assertFalse((root / "Cargo.lock").exists())

    def test_command_timeout_is_reported(self):
        """End a slow probe rather than block the complete inventory.
        Return failure data that the caller can report as broken readiness.
        """
        import subprocess
        with patch.object(CHECKER.subprocess, "run", side_effect=subprocess.TimeoutExpired(["tool"], 1)):
            result = CHECKER.run_command(["tool"], Path.cwd(), 1)
        self.assertNotEqual(result[0], 0)


if __name__ == "__main__":
    unittest.main()
