#!/usr/bin/env python3
"""Report Rust tool and project dependency readiness without mutations.
Use --online for release checks and --project for manifest/lockfile inventory.
"""

import argparse
import json
import os
from pathlib import Path
import platform
import re
import shutil
import subprocess
import sys
import tomllib
from urllib.request import Request, urlopen

# List optional tool packages and their public version probes.
TOOLS = {
    "cargo-audit": ["cargo", "audit", "--version"],
    "bacon": ["bacon", "--version"],
    "cargo-nextest": ["cargo", "nextest", "--version"],
    "flamegraph": ["cargo", "flamegraph", "--version"],
    "hyperfine": ["hyperfine", "--version"],
    "tokio-console": ["tokio-console", "--version"],
    "oha": ["oha", "--version"],
}
# Track library declarations separately from executable installations.
LIBRARIES = ("dhat", "tracing", "tracing-subscriber", "tracing-chrome",
             "console-subscriber", "tokio", "rayon", "dashmap", "rustfix")
# Accept only complete stable versions for release comparisons.
STABLE_VERSION = re.compile(r"^(\d+)\.(\d+)\.(\d+)(?:\+[0-9A-Za-z.-]+)?$")


def stable_version(value):
    """Parse a stable semantic version for ordered comparison.
    Return None for unsupported or prerelease values rather than guessing.
    """
    match = STABLE_VERSION.fullmatch(value)
    return tuple(map(int, match.groups())) if match else None


def probe_version(output):
    """Extract a version from an executable's version response.
    Preserve prerelease suffixes so they cannot be mistaken for stable releases.
    """
    match = re.search(r"\b\d+\.\d+\.\d+(?:-[0-9A-Za-z.-]+)?(?:\+[0-9A-Za-z.-]+)?", output)
    return match.group(0) if match else None


def run_command(command, cwd, timeout):
    """Run an argument-vector probe with a bounded timeout.
    Disable rustup auto-install and return failure data instead of raising.
    """
    env = dict(os.environ, RUSTUP_AUTO_INSTALL="0", CARGO_TERM_COLOR="never")
    try:
        result = subprocess.run(command, cwd=cwd, env=env, text=True,
                                stdout=subprocess.PIPE, stderr=subprocess.PIPE, timeout=timeout)
        return result.returncode, result.stdout.strip(), result.stderr.strip()
    except (OSError, subprocess.TimeoutExpired) as error:
        return 1, "", str(error)


def fetch_text(url, timeout):
    """Fetch public release metadata without writing a cache.
    Bound response size and reject oversized responses before parsing.
    """
    request = Request(url, headers={"User-Agent": "rust-instructions-environment-check/1.0"})
    with urlopen(request, timeout=timeout) as response:
        raw = response.read(16 * 1024 * 1024 + 1)
    if len(raw) > 16 * 1024 * 1024:
        raise ValueError("release response exceeds size limit")
    return raw.decode("utf-8")


def latest_crate(package, timeout):
    """Find the newest non-yanked stable crates.io release.
    Use sparse-index records to avoid assuming that maximum releases are stable.
    """
    if len(package) == 1:
        prefix = "1"
    elif len(package) == 2:
        prefix = "2"
    elif len(package) == 3:
        prefix = "3/" + package[0]
    else:
        prefix = package[:2] + "/" + package[2:4]
    entries = [json.loads(line) for line in
               fetch_text(f"https://index.crates.io/{prefix}/{package}", timeout).splitlines() if line]
    versions = [item["vers"] for item in entries
                if not item.get("yanked", False) and stable_version(item["vers"]) is not None]
    if not versions:
        raise ValueError("no non-yanked stable release found")
    return max(versions, key=stable_version)


def release_status(installed, latest, newer_label):
    """Compare stable installed and published versions conservatively.
    Leave prerelease or unrecognized versions unknown and preserve newer local builds.
    """
    current = stable_version(installed or "")
    published = stable_version(latest or "")
    if current is None or published is None:
        return "UNKNOWN"
    if current < published:
        return newer_label
    return "CURRENT" if current == published else "LOCAL_NEWER"


def tool_row(name, command, cwd, args, package=None, required=False):
    """Build one presence and usability result for a public CLI.
    Compare independent packages online; leave bundled component versions tied to Rust.
    """
    code, output, error = run_command(command, cwd, args.timeout)
    row = {"name": name, "kind": "tool", "required": required,
           "status": "PRESENT" if code == 0 else "BROKEN", "version": probe_version(output),
           "probe": command, "latest_status": "UNKNOWN"}
    if not shutil.which(command[0]):
        row["status"] = "MISSING"
    if code:
        row["detail"] = (error or output or "probe failed")[-300:]
        if package and ("no such command" in (output + error).lower()
                        or not shutil.which(command[0])):
            row["status"] = "MISSING"
    if package:
        if args.online:
            try:
                row["latest"] = latest_crate(package, args.timeout)
                row["latest_status"] = (release_status(row["version"], row["latest"], "OUTDATED")
                                        if row["status"] == "PRESENT" else "AVAILABLE")
            except (OSError, ValueError, KeyError) as failure:
                row["release_error"] = str(failure)
        if row["status"] in ("MISSING", "BROKEN") or row["latest_status"] == "OUTDATED":
            row["offer"] = f"cargo install --locked {package}; preserve existing package-manager method"
    return row


def project_rows(manifest, cwd, args):
    """Read declared workspace dependencies and locked registry versions.
    Use locked offline metadata and report uncertainty without resolving or updating dependencies.
    """
    command = ["cargo", "metadata", "--format-version", "1", "--no-deps", "--offline", "--locked",
               "--manifest-path", str(manifest)]
    code, output, error = run_command(command, cwd, args.timeout)
    rows = []
    declarations = {name: [] for name in LIBRARIES}
    root = manifest.parent
    metadata_ok = False
    if code == 0:
        try:
            metadata = json.loads(output)
            root = Path(metadata["workspace_root"])
            members = set(metadata["workspace_members"])
            for package in metadata["packages"]:
                if package["id"] not in members:
                    continue
                rows.append({"name": package["name"], "kind": "workspace-package",
                             "status": "DECLARED", "msrv": package.get("rust_version"),
                             "edition": package.get("edition")})
                for dependency in package["dependencies"]:
                    if dependency["name"] in declarations:
                        declarations[dependency["name"]].append({
                            "member": package["name"], "requirement": dependency["req"],
                            "optional": dependency.get("optional", False),
                            "features": dependency.get("features", []),
                            "kind": dependency.get("kind") or "normal", "source": dependency.get("source"),
                            "path": dependency.get("path"), "target": dependency.get("target")})
            metadata_ok = True
        except (ValueError, KeyError, TypeError) as failure:
            error = str(failure)
    if not metadata_ok:
        rows.append({"name": "cargo-metadata", "kind": "project", "status": "UNKNOWN",
                     "detail": (error or output or "metadata unavailable")[-300:]})
    lock_path = root / "Cargo.lock"
    locked = []
    lock_ok = False
    if lock_path.is_file():
        try:
            with lock_path.open("rb") as handle:
                locked = tomllib.load(handle).get("package", [])
            lock_ok = True
        except (OSError, tomllib.TOMLDecodeError) as failure:
            rows.append({"name": "Cargo.lock", "kind": "project", "status": "UNKNOWN",
                         "detail": str(failure)})
    else:
        rows.append({"name": "Cargo.lock", "kind": "project", "status": "UNKNOWN",
                     "detail": "No lockfile found; no resolution or download was attempted."})
    for name in LIBRARIES:
        found = [item for item in locked if item["name"] == name]
        row = {"name": name, "kind": "library", "declarations": declarations[name],
               "locked": [{"version": item["version"], "source": item.get("source")} for item in found],
               "status": "LOCKED" if found else "DECLARED" if declarations[name]
               else "NOT_DECLARED" if metadata_ok else "UNKNOWN", "latest_status": "UNKNOWN"}
        if declarations[name] and not lock_ok:
            row["detail"] = "Declared dependency; resolved version not verified."
        if args.online:
            try:
                row["latest"] = latest_crate(name, args.timeout)
                registry_versions = [item["version"] for item in found
                                     if item.get("source") in (
                                         "registry+https://github.com/rust-lang/crates.io-index",
                                         "sparse+https://index.crates.io/")]
                if registry_versions:
                    states = [release_status(version, row["latest"], "NEWER_AVAILABLE")
                              for version in registry_versions]
                    row["latest_status"] = "NEWER_AVAILABLE" if "NEWER_AVAILABLE" in states else (
                        "UNKNOWN" if "UNKNOWN" in states else states[0])
                    if row["latest_status"] == "NEWER_AVAILABLE":
                        row["offer"] = (f"Review constraints/MSRV before cargo update -p {name}; "
                                        "check each locked version and source")
            except (OSError, ValueError, KeyError) as failure:
                row["release_error"] = str(failure)
        rows.append(row)
    return rows


def main():
    """Collect a read-only inventory and print actions that can be approved.
    Return failure only for unusable core tools; leave optional findings visible in output.
    """
    parser = argparse.ArgumentParser(description=__doc__)
    location = parser.add_mutually_exclusive_group()
    location.add_argument("--project", type=Path)
    location.add_argument("--manifest-path", type=Path)
    parser.add_argument("--online", action="store_true")
    parser.add_argument("--json", action="store_true")
    parser.add_argument("--timeout", type=float, default=10)
    args = parser.parse_args()
    if args.timeout <= 0 or args.timeout > 60:
        parser.error("--timeout must be greater than 0 and at most 60 seconds")
    manifest = args.manifest_path
    if args.project:
        manifest = args.project / "Cargo.toml"
    if manifest:
        manifest = manifest.expanduser().resolve()
        if not manifest.is_file():
            parser.error("project manifest does not exist")
    cwd = manifest.parent if manifest else Path.cwd()
    rows = []
    probes = [("rustup", ["rustup", "--version"], False),
              ("rustc", ["rustc", "--version"], True), ("cargo", ["cargo", "--version"], True),
              ("rustfmt", ["cargo", "fmt", "--version"], True),
              ("clippy", ["cargo", "clippy", "--version"], True),
              ("rustfix (cargo fix)", ["cargo", "fix", "--help"], True)]
    for name, command, required in probes:
        row = tool_row(name, command, cwd, args, required=required)
        if name in ("rustfmt", "clippy") and row["status"] != "PRESENT":
            row["offer"] = f"rustup component add {name}; target the selected toolchain"
        elif name in ("rustc", "cargo", "rustfix (cargo fix)") and row["status"] != "PRESENT":
            row["offer"] = "Repair/install the project-selected Rust toolchain; preserve its pin and MSRV"
        rows.append(row)
    code, active, error = run_command(["rustup", "show", "active-toolchain"], cwd, args.timeout)
    rows.append({"name": "active-toolchain", "kind": "toolchain", "status": "PRESENT" if not code else "UNKNOWN",
                 "detail": active if not code else error[-300:]})
    if args.online:
        try:
            data = tomllib.loads(fetch_text("https://static.rust-lang.org/dist/channel-rust-stable.toml",
                                           args.timeout))
            rust_release = data["pkg"]["rust"]["version"].split()[0]
            for row in rows:
                if row["name"] in ("rustc", "cargo"):
                    # Cargo CLI follows the Rust release; its internal package version differs.
                    row["latest"] = rust_release
                    row["latest_status"] = release_status(row["version"], row["latest"], "NEWER_AVAILABLE")
                    if row["latest_status"] == "NEWER_AVAILABLE":
                        row["offer"] = "Offer rustup update stable only if allowed; preserve project pin and MSRV"
            rustup = rows[0]
            tag = tomllib.loads(fetch_text("https://static.rust-lang.org/rustup/release-stable.toml",
                                           args.timeout))["version"]
            rustup["latest"] = tag
            rustup["latest_status"] = release_status(rustup["version"], tag, "OUTDATED")
            if rustup["latest_status"] == "OUTDATED":
                rustup["offer"] = "Offer rustup self update; preserve existing package-manager method"
        except (OSError, ValueError, KeyError) as failure:
            rows.append({"name": "toolchain-release-check", "kind": "release", "status": "UNKNOWN",
                         "detail": str(failure)})
    for package, command in TOOLS.items():
        rows.append(tool_row(package, command, cwd, args, package=package))
    for row in rows:
        if row["name"] in ("rustfmt", "clippy", "rustfix (cargo fix)"):
            row["latest_status"] = "BUNDLED"
            row["version_policy"] = "Version follows the selected Rust toolchain; review the rustc release row."
    if platform.system() == "Darwin":
        code, output, error = run_command(["xcrun", "--find", "xctrace"], cwd, args.timeout)
        rows.append({"name": "xctrace", "kind": "profiler-backend",
                     "status": "PRESENT" if not code else "MISSING", "detail": output or error[-300:]})
    elif platform.system() == "Linux":
        rows.append(tool_row("perf", ["perf", "--version"], cwd, args))
    if manifest:
        rows.extend(project_rows(manifest, cwd, args))
    else:
        rows.extend({"name": name, "kind": "library", "status": "UNKNOWN",
                     "detail": "Use --project to check manifest declarations and Cargo.lock."} for name in LIBRARIES)
    report = {"online": args.online, "project": str(manifest) if manifest else None, "rows": rows,
              "offer": "Offer the listed installs/updates; apply only authorized changes after compatibility review."}
    if args.json:
        print(json.dumps(report, indent=2))
    else:
        for row in rows:
            versions = row.get("version") or ", ".join(item["version"] for item in row.get("locked", []))
            print(f"{row['status']}: {row['name']}" + (f" {versions}" if versions else ""))
            if row.get("latest"):
                print(f"  {row['latest_status']}: latest stable {row['latest']}")
            elif row.get("kind") == "tool":
                print("  UNKNOWN: latest version not verified; bundled components follow selected Rust")
            for field in ("detail", "version_policy", "release_error", "offer"):
                if row.get(field):
                    print(f"  {field}: {row[field]}")
            for declaration in row.get("declarations", []):
                print(f"  {declaration['member']}: {declaration['requirement']} "
                      f"({declaration['kind']}, optional={declaration['optional']})")
        print(report["offer"])
    return int(any(row.get("required") and row["status"] != "PRESENT" for row in rows))


if __name__ == "__main__":
    sys.exit(main())
