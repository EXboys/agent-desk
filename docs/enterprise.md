# Enterprise control plane (optional)

Agent Doctor works **standalone** for local discovery and `doctor` checks, but its primary product direction is team and enterprise operations: diagnose employee machines, repair broken agent runtimes, verify gateway/profile compliance, and produce auditable local reports.

Teams that run a central gateway / skill market / policy service can wire Agent Doctor to it via `setup`, `sync`, `policy pull`, and future repair policy evaluation.

## Division of responsibility

| Layer | Example | Responsibility |
|-------|---------|----------------|
| Control plane | [Evotown](https://github.com/EXboys/evotown), custom gateway | Accounts, SkillHub, policies, approved profiles, audit ingestion |
| Local client | **Agent Doctor** (this repo) | Discover runtimes, diagnose drift, back up configs, repair to team baseline |
| Runtimes | OpenClaw, Hermes, Claude Code, … | Execute tasks locally |

## Employee flow (target)

1. IT deploys a control plane and issues API keys.
2. Employee installs Agent Doctor.
3. Employee runs `agent-doctor setup --url $GATEWAY_URL --key ...`.
4. `agent-doctor doctor` shows installed runtimes, gateway wiring, and local config drift.
5. Optional: `agent-doctor sync` for private skills; `agent-doctor policy pull` for cached rules.
6. If a runtime breaks or drifts, IT runs `agent-doctor repair <runtime>` to back up, diagnose, apply approved fixes, verify, and export an audit report.

## Enterprise repair guarantees

- Diagnostics are local-first and redacted before AI analysis.
- Repair execution is limited to typed, whitelisted actions.
- Writes require confirmation and a backup snapshot.
- Reports record redacted inputs, selected actions, verification results, and rollback hints.
- Policy can disallow log upload, raw config upload, destructive actions, or non-company gateways.

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
