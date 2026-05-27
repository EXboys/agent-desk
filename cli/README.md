# CLI (`agent-desk`)

Rust binary in the Agent Desk workspace. Shared logic lives in `../crates/agent-desk-core/`.

## Build & run

```bash
# from repo root
cargo build -p agent-desk
cargo run -p agent-desk -- doctor
cargo run -p agent-desk -- doctor --json
```

## Commands

| Command | Status |
|---------|--------|
| `doctor` | Implemented (OpenClaw, Hermes, Claude Code discovery) |
| `profile list/init/use` | Implemented (Hermes model switching) |
| `config show` | Implemented (Hermes) |
| `setup` | Stub |
| `sync` | Stub |
| `policy pull` | Stub |

## Adapters

Runtime-specific code is in `crates/agent-desk-core/src/adapters/`. See [../adapters/README.md](../adapters/README.md) for the adapter contract.
