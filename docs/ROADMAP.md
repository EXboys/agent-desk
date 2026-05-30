# Roadmap

## P0 — CLI MVP

- [x] `agent-doctor doctor` — detect OpenClaw, Hermes, Claude Code, Codex; print config paths and gateway wiring
- [x] `agent-doctor doctor --explain` — AI diagnosis for runtimes with probe issues
- [x] `agent-doctor install <runtime>` — rule-based Hermes/OpenClaw install with install logs
- [x] `agent-doctor install <runtime> --explain` — install + AI failure/success explanation
- [x] Hermes model presets — `profile init/list/use`, `config show hermes`
- [x] Repair safety foundation — diagnostic sensitivity classes, redaction, typed actions, backup/audit report types
- [x] Runtime-specific read-only probes — binary, version, PATH conflicts, config parse/schema, env conflicts, gateway connectivity, MCP/Skills path references
- [x] Hermes deep probes — model schema, provider API key env, `.env` parse, duplicate/empty key checks, and `.env` file permissions
- [x] `agent-doctor repair <runtime>` — read-only probe + safe repair preview
- [x] `agent-doctor repair <runtime> --explain` — AI diagnosis from probe (no writes)
- [x] `agent-doctor repair <runtime> --apply` — backup → Hermes rule playbook → re-probe → audit (Hermes only for auto-fix today)
- [x] `agent-doctor repair <runtime> --rollback [--backup <id>]` — restore configs from `~/.config/agent-doctor/backups/`
- [x] `agent-doctor setup` — write `~/.config/agent-doctor/profile.env` + merge runtime configs
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
| Install OpenClaw via official `openclaw.ai/install.sh` when binary missing | Shipped |
| Rollback from backup directory | Shipped (CLI + desktop) |
| AI-generated repair plans / free-form shell | Not planned for v1 |
| Auto-fill or upload API keys | Not planned |
| `install` / `update` runtime binaries | Hermes + OpenClaw: `install` command + repair playbook; Claude/Codex planned |
| OpenClaw / Claude / Codex rule playbooks | OpenClaw: install + config/schema/env/gateway fixes shipped; Claude/Codex planned |

## P1 — Desktop tray

- [x] Tauri menubar shell: tray menu, left-click to show window, run doctor
- [x] Runtime diagnosis panel with filterable checks and suggested fixes
- [x] Hermes: apply fixes, rollback, open API key guide
- [ ] Keychain storage for API keys (optional)

## P2 — Project workspaces

- [x] `agent-doctor workspace init` — register project; bind Hermes profile, Claude project dir, Codex CODEX_HOME, OpenClaw agent workspace
- [x] `agent-doctor workspace use` — activate workspace + write `active-workspace.env`
- [x] `agent-doctor workspace status` / `workspace doctor` — alignment and memory bleed checks
- [x] `agent-doctor workspace enter` / `env` / `match` — shell eval exports
- [x] `agent-doctor workspace hook install` — zsh/bash cd auto-align
- [x] `workspace use` backups before switching (Hermes/OpenClaw/project Claude settings)
- [x] MCP/skills snapshot per workspace (`workspaces/<name>/snapshots/`)
- [x] `workspace fix` — auto-rebind Hermes/OpenClaw, refresh env, restore project `.mcp.json`
- [x] Claude `~/.claude.json` user-scoped MCP detection in workspace doctor
- [x] Shell `cd` hook / direnv integration (fish + `workspace direnv`)
- [x] Desktop tray workspace switch + tooltip
- [x] `workspace use --restart-gateways` — Hermes/OpenClaw gateway restart

- [ ] Local policy evaluate before ingest
- [x] OpenClaw install via repair loop / playbook when `openclaw` binary missing
- [x] OpenClaw repair playbook: config schema, env permissions, gateway from profile, API key scaffold
- [ ] OpenClaw MCP/skills symlink checks (path reference auto-fix)
- [ ] Team baseline drift detection for gateway, model provider, MCP, and Skill settings
- [ ] SkillLite adapter (optional runtime)

## Optional integrations

- [Evotown](https://github.com/EXboys/evotown) — see [enterprise.md](enterprise.md)
