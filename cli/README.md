# CLI (`agent-doctor`)

Rust binary in the Agent Doctor workspace. Shared logic lives in `../crates/agent-doctor-core/`.

Agent Doctor is CLI-first so it can be used from onboarding scripts, IT support playbooks, and future repair automation.

## Build & run

```bash
# from repo root
cargo build -p agent-doctor
cargo run -p agent-doctor -- doctor
cargo run -p agent-doctor -- doctor --json
```

## Commands

| Command | Status |
|---------|--------|
| `doctor` | Implemented (OpenClaw, Hermes, Claude Code, Codex discovery); `--explain` for AI diagnosis |
| `install <runtime>` | All registered runtimes: rule install when available; else / on failure → AI install |
| `profile list/init/use` | Implemented (Hermes model switching) |
| `config show` | Implemented (Hermes) |
| `repair <runtime>` | Probes + preview; Hermes/OpenClaw auto-fix with `--apply`; `--explain` for AI diagnosis |
| `setup` | Stub |
| `sync` | Stub |
| `policy pull` | Stub |

## Adapters

Runtime-specific code is in `crates/agent-doctor-core/src/adapters/`. See [../adapters/README.md](../adapters/README.md) for the adapter contract.

Repair safety primitives live in `agent-doctor-core::repair`: diagnostic facts are classified by sensitivity, redacted before AI analysis, and converted into typed repair actions before execution. Runtime probes live in `agent-doctor-core::probe` and currently check binaries, versions, PATH conflicts, config parse/schema issues, env conflicts, gateway reachability, and obvious MCP/Skills path references.
