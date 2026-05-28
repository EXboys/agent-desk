# Roadmap

## P0 — CLI MVP

- [x] `agent-doctor doctor` — detect OpenClaw, Hermes, Claude Code, Codex; print config paths and gateway wiring
- [x] Hermes model presets — `profile init/list/use`, `config show hermes`
- [ ] `agent-doctor repair <runtime>` — back up configs, diagnose common breakages, and apply confirmed fixes
- [ ] `agent-doctor setup` — write `~/.config/agent-doctor/profile.env` + merge runtime configs
- [ ] `agent-doctor sync` — skill bundle sync from control plane
- [ ] `agent-doctor policy pull` — cache policies from control plane
- [ ] Company profile: `--url` + API key

## P1 — Desktop tray

- [x] Tauri menubar shell: tray menu, left-click to show window, run doctor
- [ ] Keychain storage for API keys (optional)

## P2 — Adapters & policy

- [ ] Local policy evaluate before ingest
- [ ] OpenClaw repair playbook: install health, config schema, env, MCP/skills symlink checks
- [ ] SkillLite adapter (optional runtime)

## Optional integrations

- [Evotown](https://github.com/EXboys/evotown) — see [enterprise.md](enterprise.md)
