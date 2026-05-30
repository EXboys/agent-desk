# Project workspaces

Agent Doctor **workspaces** isolate agent runtime state per project. This is separate from **profiles** (company gateway / model presets).

Supported runtimes: Hermes, Claude Code, Codex, OpenClaw.

## Quick start

```bash
cd ~/projects/my-app
agent-doctor workspace init          # register project (optional: --git-root, --name)
agent-doctor workspace use my-app    # activate + bind runtimes
eval "$(agent-doctor workspace env --shell zsh --name my-app)"
cd ~/projects/my-app
```

Or one step from the project directory:

```bash
agent-doctor workspace enter
eval "$(agent-doctor workspace env --shell zsh)"
```

## What gets isolated

| Runtime | Mechanism | Tier |
|---------|-----------|------|
| Hermes | Dedicated profile + `HERMES_HOME` | L3 |
| Claude Code | Project `.claude/` + `.mcp.json` | L3 |
| Codex | Per-workspace `CODEX_HOME` | L2 |
| OpenClaw | Per-workspace agent workspace in `openclaw.json` | L2 |

Config files:

- `~/.config/agent-doctor/workspaces.yaml` — registry
- `~/.config/agent-doctor/active-workspace.env` — env exports
- `~/.config/agent-doctor/workspaces/<name>/` — Codex home, OpenClaw workspace, MCP/skills snapshots

## MCP & skills snapshots

On `workspace init` / `workspace use`, Agent Doctor:

1. Saves project `.mcp.json` and `.claude/skills/` to `workspaces/<name>/snapshots/`
2. Restores them to the project if missing (e.g. fresh clone)

Prefer project-scoped `.mcp.json` over user-scoped MCP in `~/.claude/settings.json` or `~/.claude.json`.

## Shell integration

Auto-align env when you `cd` into a registered project:

```bash
agent-doctor workspace hook install          # zsh + bash + fish
agent-doctor workspace hook install --shell fish
source ~/.config/agent-doctor/hooks/workspace.fish   # add to config.fish
```

### direnv

```bash
agent-doctor workspace direnv --name my-app        # print .envrc
agent-doctor workspace direnv --name my-app --write  # write project/.envrc
direnv allow
```

## Gateway restart

When Hermes gateway is running under a different profile:

```bash
agent-doctor workspace use my-app --restart-gateways
agent-doctor workspace fix --restart-gateways
```

## Diagnostics & repair

```bash
agent-doctor workspace status
agent-doctor workspace doctor
agent-doctor workspace fix --dry-run   # preview
agent-doctor workspace fix             # re-bind Hermes/OpenClaw, refresh env, restore .mcp.json
```

`workspace use` creates a backup before switching (Hermes profile, OpenClaw config, project Claude settings). Skip with `--no-backup`.

## Remove

```bash
agent-doctor workspace remove my-app
agent-doctor workspace remove my-app --purge   # delete workspace data dir
```

Does not delete Hermes profiles or OpenClaw agents — only the Agent Doctor registry entry.
