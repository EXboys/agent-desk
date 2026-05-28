# Roadmap

## P0 — CLI MVP

- [x] `agent-doctor doctor` — detect OpenClaw, Hermes, Claude Code, Codex; print config paths and gateway wiring
- [x] Hermes model presets — `profile init/list/use`, `config show hermes`
- [x] Repair safety foundation — diagnostic sensitivity classes, redaction, typed actions, backup/audit report types
- [ ] `agent-doctor repair <runtime>` — back up configs, diagnose common breakages, and apply confirmed fixes
- [ ] `agent-doctor setup` — write `~/.config/agent-doctor/profile.env` + merge runtime configs
- [ ] `agent-doctor sync` — skill bundle sync from control plane
- [ ] `agent-doctor policy pull` — cache policies from control plane
- [ ] Company profile: `--url` + API key
- [ ] Compliance report export for IT / DevEx support workflows

## P1 — Desktop tray

- [x] Tauri menubar shell: tray menu, left-click to show window, run doctor
- [ ] Keychain storage for API keys (optional)

## P2 — Adapters & policy

- [ ] Local policy evaluate before ingest
- [ ] OpenClaw repair playbook: install health, config schema, env, MCP/skills symlink checks
- [ ] Team baseline drift detection for gateway, model provider, MCP, and Skill settings
- [ ] SkillLite adapter (optional runtime)

## Optional integrations

- [Evotown](https://github.com/EXboys/evotown) — see [enterprise.md](enterprise.md)
