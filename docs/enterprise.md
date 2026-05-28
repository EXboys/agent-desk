# Enterprise control plane (optional)

Agent Doctor works **standalone** for local discovery and `doctor` checks. Teams that run a central gateway / skill market / policy service can wire Agent Doctor to it via `setup`, `sync`, and `policy pull`.

## Division of responsibility

| Layer | Example | Responsibility |
|-------|---------|----------------|
| Control plane | [Evotown](https://github.com/EXboys/evotown), custom gateway | Accounts, SkillHub, policies, audit |
| Local client | **Agent Doctor** (this repo) | Discover runtimes, apply profile, sync skills |
| Runtimes | OpenClaw, Hermes, Claude Code, … | Execute tasks locally |

## Employee flow (target)

1. IT deploys a control plane and issues API keys.
2. Employee installs Agent Doctor.
3. Employee runs `agent-doctor setup --url $GATEWAY_URL --key ...`.
4. `agent-doctor doctor` shows installed runtimes and gateway wiring.
5. Optional: `agent-doctor sync` for private skills; `agent-doctor policy pull` for cached rules.

## Evotown example

[Evotown](https://github.com/EXboys/evotown) is one supported control plane:

| API | Purpose |
|-----|---------|
| `GET /api/v1/market/bundles/.../manifest` | Skill sync |
| `GET /api/v1/policies` | Policy cache |
| `POST /api/v1/policy/evaluate` | Pre-flight checks (planned) |

Legacy setup script (to be superseded by Agent Doctor):

```bash
python3 /path/to/evotown/scripts/evotown-agent-setup.py check
python3 /path/to/evotown/scripts/evotown-agent-setup.py sync
```

Release downloads: `https://github.com/EXboys/agent-doctor/releases/latest`
