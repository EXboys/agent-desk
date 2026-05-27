# Agent Desk

**Manage desktop AI agents on one machine** — discover what's installed, where configs live, and apply a company profile in one command.

[License: MIT](LICENSE)

---

## Why Agent Desk?

Developers often run **several** local agents at once:


| Runtime                                                       | Typical config              |
| ------------------------------------------------------------- | --------------------------- |
| [OpenClaw](https://github.com/openclaw/openclaw)              | `~/.openclaw/openclaw.json` |
| [Hermes Agent](https://github.com/nousresearch/hermes-agent)  | `~/.hermes/config.yaml`     |
| [Claude Code](https://docs.anthropic.com/en/docs/claude-code) | `~/.claude/settings.json`   |
| Codex CLI                                                     | provider env / config       |


Each tool has its own install path, gateway settings, and skills manifest. Agent Desk gives you **one** place to answer:

- What is installed on this laptop?
- Where do configs live?
- Are runtimes pointed at the right gateway?
- Can we apply or verify a team profile before agents run?

```text
  Your laptop
 ┌─────────────────────────┐
 │ Agent Desk              │
 │ doctor · setup · sync   │
 └───────────┬─────────────┘
             │
   OpenClaw · Hermes · Claude Code · Codex
```

Optional: teams can plug in an enterprise control plane (e.g. [Evotown](https://github.com/EXboys/evotown)) for gateway keys, SkillHub, and policy — see [docs/enterprise.md](docs/enterprise.md).

---

## Status

🚧 **Early MVP** — Rust workspace + `agent-desk doctor` + Tauri menubar shell. See [docs/ROADMAP.md](docs/ROADMAP.md) for remaining P0 items (`setup`, `sync`, `policy pull`).

---

## Planned commands

```bash
# Discover installed runtimes, config paths, gateway wiring
agent-desk doctor

# Apply company profile (URL + API key + per-runtime config)
agent-desk setup --url https://gateway.company.internal --key ...

# Pull private skill bundle from control plane
agent-desk sync

# Cache policies from control plane
agent-desk policy pull
```

---

## Relationship to other tools


| Project                                                     | Scope                                                                         |
| ----------------------------------------------------------- | ----------------------------------------------------------------------------- |
| **[ClawPanel](https://github.com/qingchencloud/clawpanel)** | Rich GUI for OpenClaw + Hermes                                                |
| **[ClawPal](https://github.com/lay2dev/clawpal)**           | OpenClaw desktop config companion                                             |
| **Agent Desk**                                              | **Cross-runtime local discovery + profile setup** (not a replacement runtime) |


---

## 中文

**Agent Desk（本机 Agent 工作台）** 用于在同一台电脑上**发现**已安装的 OpenClaw、Hermes、Claude Code、Codex 等，**查看配置路径**，并**一键应用**团队网关/Skill 配置模板。

详见 [docs/zh-CN/README.md](docs/zh-CN/README.md)。企业控制面集成（可选）见 [docs/enterprise.md](docs/enterprise.md)。

---

## Development

```bash
# CLI
cargo run -p agent-desk -- doctor

# Local CI checks (fmt / clippy / test)
make check
# or: ./scripts/check.sh cli

# Desktop menubar (requires Node.js)
cd desktop && npm install && npm run tauri dev
```

See [docs/development.md](docs/development.md), [docs/ROADMAP.md](docs/ROADMAP.md), [docs/install.md](docs/install.md), [cli/README.md](cli/README.md), [desktop/README.md](desktop/README.md), and [CONTRIBUTING.md](CONTRIBUTING.md).

## Install

Prebuilt CLI and desktop bundles are published to [GitHub Releases](https://github.com/EXboys/agent-desk/releases).

```bash
# Latest CLI (pick the pattern for your OS — see docs/install.md)
gh release download --repo EXboys/agent-desk --pattern 'agent-desk-*-macos-arm64.tar.gz'
tar -xzf agent-desk-*-macos-arm64.tar.gz && chmod +x agent-desk
./agent-desk doctor
```

## License

MIT — see [LICENSE](LICENSE).