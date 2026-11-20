# Linux and macOS compatibility CI

The [Unix platform compatibility workflow](../.github/workflows/unix-platforms.yml) checks Soro Mutants on GitHub-hosted Ubuntu and macOS runners on pull requests, pushes to main, and manual dispatch.

Each independent matrix job uses current stable Rust, builds the CLI with the committed lockfile, runs all normal unit/integration tests, verifies the machine-readable platform policy, and checks that repeated semantic discovery on `fixtures/weak-auth` produces byte-identical JSON. The CLI also exercises the count-only discovery path. Report and runner regression tests are included in the full test suite; the workflow does **not** run unbounded mutation campaigns or external benchmarks.

The jobs are limited to 20 minutes each. A failure on one operating system does not cancel the other.

## Local reproduction

On either supported host, from the repository root:

```bash
cargo build --locked
cargo test --locked --all-targets --all-features
cargo run --locked -- check-platform --json
target/debug/cargo-soro-mutants list fixtures/weak-auth --json > first.json
target/debug/cargo-soro-mutants list fixtures/weak-auth --json > second.json
cmp first.json second.json
target/debug/cargo-soro-mutants list fixtures/weak-auth --count-only
```

Delete the two temporary JSON files afterward. On macOS, ensure the Xcode command-line tools and a supported Rust toolchain are installed. If discovery differs, attach both JSON outputs and the `rustc --version` output to the issue. This workflow does not change mutation semantics or CLI contracts.
