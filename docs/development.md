# Development

## Local checks (match CI)

The workspace includes **CLI crates** and a **Tauri desktop crate**. On Linux, the desktop crate needs GTK/WebKit system libraries; CI therefore splits checks by platform.

### Model presets (Hermes)

```bash
agent-desk profile init          # create ~/.config/agent-desk/profiles.yaml
agent-desk profile list
agent-desk profile use work      # writes Hermes config + backup
agent-desk config show hermes
```

Edit `~/.config/agent-desk/profiles.yaml` to add your own presets. Example: [profiles.example.yaml](../examples/profiles.example.yaml).

### Quick commands

```bash
# Same as CI on any OS (CLI + frontend)
make check
# or
./scripts/check.sh cli && ./scripts/check.sh frontend

# Full local sweep on macOS (includes desktop Rust)
./scripts/check.sh all

# Format fix
make fmt
# or
./scripts/check.sh fmt
```

### Cargo aliases

```bash
cargo check-cli   # clippy core + CLI
cargo test-cli    # test core + CLI
```

### Config files

| File | Purpose |
|------|---------|
| `rustfmt.toml` | rustfmt style |
| `clippy.toml` | clippy settings |
| `.cargo/config.toml` | cargo aliases |

## Linux: desktop Rust builds

If you see `Package 'glib-2.0' not found` when building `agent-desk-desktop`:

```bash
sudo apt-get update
sudo apt-get install -y \
  libwebkit2gtk-4.1-dev \
  libappindicator3-dev \
  librsvg2-dev \
  patchelf \
  pkg-config
```

Then:

```bash
AGENT_DESK_CHECK_DESKTOP=1 ./scripts/check.sh desktop
```

## CI layout

| Job | Runner | Scope |
|-----|--------|-------|
| `rust-cli` | ubuntu-latest | fmt, clippy, test, build for `agent-desk-core` + `agent-desk` |
| `rust-desktop` | macos-latest | clippy for `agent-desk-desktop` |
| `desktop-frontend` | ubuntu-latest | `npm ci && npm run build` |

Release builds install Linux GUI dependencies only in the desktop release job.
