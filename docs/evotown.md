# Evotown integration

Agent Desk is the **recommended local client** for employee machines connecting to an [Evotown](https://github.com/EXboys/evotown) deployment.

## Division of responsibility

| Layer | Repository | Responsibility |
|-------|------------|----------------|
| Control plane | `EXboys/evotown` | Gateway, accounts (`evk_`), SkillHub, policies, runs, console |
| Local client | `EXboys/agent-desk` | Discover runtimes, apply profile, sync skills, policy cache |
| Runtimes | OpenClaw, Hermes, Claude Code, … | Execute tasks locally |

## Employee flow (target)

1. IT deploys Evotown (`scripts/enterprise-deploy.sh`) and issues `evk_` keys.
2. Employee installs Agent Desk (release binary or package).
3. Employee runs `agent-desk setup --url $EVOTOWN_URL --key evk_...`.
4. `agent-desk doctor` shows all runtimes and whether they use the company gateway.
5. Optional: `agent-desk sync` for private skills; IT enables connector separately with `evi_`.

## APIs used

| API | Purpose |
|-----|---------|
| `GET /api/v1/market/bundles/.../manifest` | Skill sync |
| `GET /api/v1/policies` | Policy cache |
| `POST /api/v1/policy/evaluate` | Pre-flight checks (planned) |
| `POST /api/v1/events` | Via connector / ingest token (`evi_`) |

## Console linkage

Evotown web console (`/market`, `/accounts`) should link to Agent Desk releases:

`https://github.com/EXboys/agent-desk/releases/latest`
