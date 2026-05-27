# Runtime adapters

Each adapter implements:

| Method | Description |
|--------|-------------|
| `discover()` | Is runtime installed? Version? Binary path? |
| `config_paths()` | Default config file locations |
| `read_profile()` | Current gateway URL / key source (redacted) |
| `apply(profile)` | Merge Evotown company template |

## Planned

| Adapter | Priority |
|---------|----------|
| `openclaw` | P0 |
| `claude-code` | P0 |
| `hermes` | P1 |
| `codex` | P1 |
| `skilllite` | P2 |
