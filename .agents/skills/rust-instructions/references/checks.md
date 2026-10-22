# Checks and hooks

## Environment checker

- Run `bash scripts/check-rust-environment.sh` from the skill directory for local machine inventory.
- Add `--project /absolute/project/path` or `--manifest-path /absolute/path/Cargo.toml` for project inventory.
- Add `--online` for current stable Rust and non-yanked stable registry release checks.
- Add `--json` for structured output; use `--timeout 10` to change each command/request timeout.
- Use Python 3.11 or newer for JSON, TOML, Cargo metadata, and release parsing; use `PYTHON` to select it.
- Read Bash's prerequisite report if Python is unavailable; do not report the full check as completed.
- Expect exit 0 for a completed inventory, 1 for missing/broken required Rust tools, and 2 for invalid
  arguments.
- Inspect every row; do not interpret exit 0 as every optional tool being installed or every version being
  current.

## Interpret status

- Treat `MISSING` as an absent executable and `BROKEN` as an executable/subcommand that failed its probe.
- Treat `OUTDATED` as a tool with a verified newer stable release, subject to installation and compatibility
  policy.
- Treat `NEWER_AVAILABLE` on compiler/library rows as availability, not permission to change a pin or
  requirement.
- Treat `UNKNOWN` as incomplete evidence, including offline checks, unsupported versions, or registry
  failures.
- Treat `NOT_DECLARED` on a library as a project fact, not a defect or instruction to add it.
- Treat `LOCKED` as Cargo.lock presence, not proof of compilation, downloaded cache, or enabled features.
- Treat `DECLARED` as a manifest dependency, not proof that an optional dependency is active.
- Review tool versions, command failures, manifest requirements, resolved versions, source kind, and suggested
  actions.

## Limits and updates

- Use the checker as a read-only inventory; do not expect it to install tools, update locks, or compile code.
- Keep rustfmt, Clippy, and cargo-fix aligned with the selected compiler; do not compare their versions to
  crate releases.
- Use the reported stable Rust release for compiler availability; preserve project pins and MSRV.
- Distinguish crates.io releases from git/path/alternate-registry dependencies; do not propose registry
  updates for them.
- Inspect compatibility before an approved `cargo update -p PACKAGE`; change manifest constraints only
  deliberately.
- Follow the existing package manager for updates; do not overwrite its executable with a Cargo installation
  blindly.
- Offer the concrete reported actions for missing/outdated items and ask which updates to perform if not
  authorized.
- Rerun the checker, supported tests, Clippy, and audit after approved changes.
- Check linker/target readiness by building supported configurations; tool presence alone does not establish
  it.
- Check profiler permissions and instrumented features with an actual diagnostic run when profiling is
  requested.

## Pre-commit and CI

- Integrate `scripts/pre-commit-rust.sh` into the existing hook manager when setting up a Rust project.
- Install all applicable project hooks; preserve existing hooks and `core.hooksPath`.
- Invoke the script from the Cargo workspace root; keep it check-only and run it for the whole workspace.
- Require rustfmt and Clippy; select supported features/targets with project-specific wrapper commands.
- Keep expanded feature/target/MSRV tests, doctests, and current RustSec audit in CI or the established hook
  workflow.
- Do not run benchmarks, watchers, profilers, or automatic fixes on every commit.
- Run `cargo fmt --all` and inspect changes before restaging; do not bypass a failing hook.
- Do not claim a custom file named `agents-instructions.md` is auto-loaded; explicitly reference it from the
  project's active agent instructions or attach it to the agent task when adopting it.
