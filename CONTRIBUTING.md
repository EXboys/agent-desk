# Contributing

Agent Doctor is in early bootstrap. Before opening large PRs, please open an issue describing:

- Which runtime adapter (OpenClaw, Hermes, Claude Code, Codex, …)
- Whether the change is local-only or requires a control-plane API

## Layout

- `crates/agent-doctor-core/` — shared discovery, doctor, company profile logic
- `cli/` — `agent-doctor` binary (Rust)
- `desktop/` — Tauri menubar app (Rust + TypeScript UI)
- `adapters/` — adapter contract docs; implementations live in `agent-doctor-core`
- `scripts/check.sh` — local fmt/clippy/test (see [docs/development.md](docs/development.md))
- `docs/` — user docs; optional enterprise integration in `enterprise.md`

## Code of conduct

Be respectful. Security issues: please report privately via GitHub Security Advisories on this repository.
