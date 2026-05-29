# Roadmap

## P0 — CLI MVP

- [x] `agent-doctor doctor` — detect OpenClaw, Hermes, Claude Code, Codex; print config paths and gateway wiring
- [x] Hermes model presets — `profile init/list/use`, `config show hermes`
- [x] Repair safety foundation — diagnostic sensitivity classes, redaction, typed actions, backup/audit report types
- [x] Runtime-specific read-only probes — binary, version, PATH conflicts, config parse/schema, env conflicts, gateway connectivity, MCP/Skills path references
- [x] Hermes deep probes — model schema, provider API key env, `.env` parse, duplicate/empty key checks, and `.env` file permissions
- [x] `agent-doctor repair <runtime>` — read-only probe + safe repair preview
- [x] `agent-doctor repair <runtime> --apply` — backup → Hermes rule playbook → re-probe → audit (Hermes only for auto-fix today)
- [x] `agent-doctor repair <runtime> --rollback [--backup <id>]` — restore configs from `~/.config/agent-doctor/backups/`
- [ ] `agent-doctor setup` — write `~/.config/agent-doctor/profile.env` + merge runtime configs
- [ ] `agent-doctor sync` — skill bundle sync from control plane
- [ ] `agent-doctor policy pull` — cache policies from control plane
- [ ] Company profile: `--url` + API key
- [ ] Compliance report export for IT / DevEx support workflows

### Hermes repair (shipped vs planned)

| Capability | Status |
|------------|--------|
| Config backup before writes | Shipped |
| `.env` permissions → `600` | Shipped |
| Duplicate API key env entries deduped | Shipped |
| Fill empty `model.*` from active profile | Shipped |
| Missing API key → `.env` placeholder + local guide (no secret fill) | Shipped |
| Install Hermes via official `install.sh` / `install.ps1` when binary missing | Shipped |
| Rollback from backup directory | Shipped (CLI + desktop) |
| AI-generated repair plans / free-form shell | Not planned for v1 |
| Auto-fill or upload API keys | Not planned |
| `install` / `update` runtime binaries | Hermes: shipped via official installer (`repair hermes --apply`); other runtimes planned |
| OpenClaw / Claude / Codex rule playbooks | Planned |

## P1 — Desktop tray

- [x] Tauri menubar shell: tray menu, left-click to show window, run doctor
- [x] Runtime diagnosis panel with filterable checks and suggested fixes
- [x] Hermes: apply fixes, rollback, open API key guide
- [ ] Keychain storage for API keys (optional)

## P2 — Adapters & policy

- [ ] Local policy evaluate before ingest
- [ ] OpenClaw repair playbook: install health, config schema, env, MCP/skills symlink checks
- [ ] Team baseline drift detection for gateway, model provider, MCP, and Skill settings
- [ ] SkillLite adapter (optional runtime)

## Optional integrations

- [Evotown](https://github.com/EXboys/evotown) — see [enterprise.md](enterprise.md)
