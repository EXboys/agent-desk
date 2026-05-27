# Contributing

Agent Desk is in early bootstrap. Before opening large PRs, please open an issue describing:

- Which runtime adapter (OpenClaw, Hermes, Claude Code, Codex, …)
- Whether the change is local-only or requires Evotown API updates

## Layout (planned)

- `cli/` — `agent-desk` binary (Rust or Go TBD)
- `desktop/` — optional Tauri tray app (calls the same core)
- `adapters/` — per-runtime discover / apply / doctor
- `docs/` — user and Evotown integration docs

## Code of conduct

Be respectful. Security issues: report privately to the Evotown maintainers if the issue affects production deployments.
