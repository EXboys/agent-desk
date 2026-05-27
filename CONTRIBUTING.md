# Contributing

Agent Desk is in early bootstrap. Before opening large PRs, please open an issue describing:

- Which runtime adapter (OpenClaw, Hermes, Claude Code, Codex, …)
- Whether the change is local-only or requires a control-plane API

## Layout

- `crates/agent-desk-core/` — shared discovery, doctor, company profile logic
- `cli/` — `agent-desk` binary (Rust)
- `desktop/` — Tauri menubar app (Rust + TypeScript UI)
- `adapters/` — adapter contract docs; implementations live in `agent-desk-core`
- `docs/` — user docs; optional enterprise integration in `enterprise.md`

## Code of conduct

Be respectful. Security issues: please report privately via GitHub Security Advisories on this repository.
