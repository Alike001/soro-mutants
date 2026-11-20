# Windows-native CLI compatibility checks

The [Windows CI workflow](../.github/workflows/windows-cli.yml) builds the CLI on `windows-latest` and runs a dedicated regression test for paths containing spaces, JSON serialization, deterministic discovery, and count-only output. It runs on PRs, pushes to main, and manual dispatch, with a 20-minute job timeout.

This is a **best-effort native Windows CLI** check, not a claim that the full mutation runner works natively. Mutation test commands still use `bash -lc`; see [supported platforms](supported-platforms.md) for the WSL2 recommendation. The smoke test deliberately uses `std::process::Command` with individual arguments, not a shell command string, so path quoting is exercised without introducing shell ambiguity.

## Reproduce on Windows PowerShell

```powershell
cargo build --locked
cargo test --locked --test windows_cli_portability -- --nocapture
.\target\debug\cargo-soro-mutants.exe list fixtures\weak-auth --count-only
```

The regression fixture is copied into a temporary directory whose name contains spaces, and removed after the assertions. If the test fails, capture the Windows Rust/Cargo versions, the failing command's stderr, and the path used. No JSON schema or mutation recognition behavior changes.
