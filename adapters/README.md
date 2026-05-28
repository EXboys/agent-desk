# Runtime adapters

Each adapter implements:

| Method | Description |
|--------|-------------|
| `discover()` | Is runtime installed? Version? Binary path? |
| `config_paths()` | Default config file locations |
| `read_profile()` | Current gateway URL / key source (redacted) |
| `apply(profile)` | Merge company profile template |

## Planned

| Adapter | Priority | Implementation |
|---------|----------|----------------|
| `openclaw` | P0 | `crates/agent-doctor-core/src/adapters/openclaw.rs` |
| `claude-code` | P0 | `crates/agent-doctor-core/src/adapters/claude_code.rs` |
| `hermes` | P0 | `crates/agent-doctor-core/src/adapters/hermes.rs` |
| `codex` | P1 | `crates/agent-doctor-core/src/adapters/codex.rs` |
| `skilllite` | P2 | — |
