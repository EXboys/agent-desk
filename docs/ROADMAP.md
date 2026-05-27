# Roadmap

## P0 — CLI MVP

- [x] `agent-desk doctor` — detect OpenClaw, Hermes, Claude Code (Codex pending); print config paths and gateway wiring
- [x] Hermes model presets — `profile init/list/use`, `config show hermes`
- [ ] `agent-desk setup` — write `~/.config/agent-desk/profile.env` + merge runtime configs
- [ ] `agent-desk sync` — skill bundle sync from control plane
- [ ] `agent-desk policy pull` — cache policies from control plane
- [ ] Company profile: `--url` + API key

## P1 — Desktop tray

- [x] Tauri menubar shell: tray menu, left-click to show window, run doctor
- [ ] Keychain storage for API keys (optional)

## P2 — Adapters & policy

- [ ] Local policy evaluate before ingest
- [ ] OpenClaw plugin install helper
- [ ] SkillLite adapter (optional runtime)

## Optional integrations

- [Evotown](https://github.com/EXboys/evotown) — see [enterprise.md](enterprise.md)
