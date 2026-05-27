# Agent Desk

**Manage desktop AI agents on one machine** — discover what's installed, where configs live, and apply a company profile in one command.

Built for teams using [Evotown](https://github.com/EXboys/evotown) as the control plane, but usable locally without Evotown for config inspection and `doctor` checks.

[![Evotown](https://img.shields.io/badge/Evotown-official%20client-blue)](https://github.com/EXboys/evotown)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

---

## Why Agent Desk?

Developers often run **several** local agents at once:

| Runtime | Typical config |
|---------|----------------|
| [OpenClaw](https://github.com/openclaw/openclaw) | `~/.openclaw/openclaw.json` |
| [Hermes Agent](https://github.com/nousresearch/hermes-agent) | `~/.hermes/config.yaml` |
| [Claude Code](https://docs.anthropic.com/en/docs/claude-code) | `~/.claude/settings.json` |
| Codex CLI | provider env / config |

Each tool has its own install path, gateway settings, and skills manifest. IT wants **one** place to answer:

- What is installed on this laptop?
- Are they pointed at the **company gateway** (`evk_` + Evotown)?
- Can we apply or verify policy before agents run?

**Agent Desk** is the local client for that job. [Evotown](https://github.com/EXboys/evotown) remains the server-side control plane (gateway, SkillHub, policies, runs, audit).

```text
  Employee laptop                         Company network
 ┌─────────────────────────┐            ┌──────────────────┐
 │ Agent Desk (this repo)  │  evk_/evi_ │ Evotown          │
 │ doctor · setup · sync   │ ─────────► │ gateway · market │
 └───────────┬─────────────┘            │ policies · runs  │
             │                          └──────────────────┘
   OpenClaw · Hermes · Claude Code · Codex
```

---

## Status

🚧 **Early bootstrap** — repository and CLI layout only. Implementation tracks [docs/ROADMAP.md](docs/ROADMAP.md).

Until the `agent-desk` binary ships, use the legacy script on the Evotown repo:

```bash
python3 /path/to/evotown/scripts/evotown-agent-setup.py check
python3 /path/to/evotown/scripts/evotown-agent-setup.py sync
```

---

## Planned commands

```bash
# Discover installed runtimes, config paths, gateway wiring
agent-desk doctor

# Apply company Evotown profile (URL + evk_ + per-runtime config)
agent-desk setup --url https://evotown.company.internal --key evk_...

# Pull private SkillHub bundle
agent-desk sync

# Cache policies from control plane
agent-desk policy pull
```

---

## Relationship to other tools

| Project | Scope |
|---------|--------|
| **[Evotown](https://github.com/EXboys/evotown)** | Enterprise control plane (server) |
| **[ClawPanel](https://github.com/qingchencloud/clawpanel)** | Rich GUI for OpenClaw + Hermes |
| **[ClawPal](https://github.com/lay2dev/clawpal)** | OpenClaw desktop config companion |
| **[agentmanager](https://github.com/kevinelliott/agentmanager)** | Install/update coding CLIs (Claude Code, Copilot, …) |
| **Agent Desk** | **Cross-runtime local setup + Evotown profile** (not a replacement runtime) |

---

## 中文

**Agent Desk（本机 Agent 工作台）** 用于在同一台电脑上**发现**已安装的 OpenClaw、Hermes、Claude Code、Codex 等，**查看配置路径**，并**一键应用**企业 [Evotown](https://github.com/EXboys/evotown) 接入模板（网关 + SkillHub + 策略拉取）。

Evotown 负责公司侧控制面；Agent Desk 负责员工笔记本电脑侧。详见 [docs/zh-CN/README.md](docs/zh-CN/README.md)。

---

## Development

See [docs/ROADMAP.md](docs/ROADMAP.md) and [CONTRIBUTING.md](CONTRIBUTING.md).

## License

MIT — see [LICENSE](LICENSE).
