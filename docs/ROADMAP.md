# Roadmap

## P0 — CLI MVP

- [ ] `agent-desk doctor` — detect OpenClaw, Hermes, Claude Code, Codex; print config paths and gateway wiring
- [ ] `agent-desk setup` — write `~/.config/evotown/evotown.agent.env` + merge runtime configs
- [ ] `agent-desk sync` — SkillHub bundle sync (port from `evotown-agent-setup.py`)
- [ ] `agent-desk policy pull` — cache `GET /api/v1/policies`
- [ ] Evotown profile: `--url` + `--key evk_`

## P1 — Desktop tray

- [ ] Tauri menubar: connected / not connected, re-sync, open Evotown console
- [ ] Keychain storage for `evk_` (optional)

## P2 — Adapters & policy

- [ ] Local policy evaluate before ingest (shared with Evotown policy engine)
- [ ] OpenClaw plugin install helper
- [ ] SkillLite adapter (optional runtime)

## Related repos

- [evotown](https://github.com/EXboys/evotown) — control plane
- Legacy script: `evotown/scripts/evotown-agent-setup.py` (to be superseded)
