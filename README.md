# Agent Doctor

**Diagnose, back up, and repair local AI agent runtimes** — find broken installs, inspect config drift, and restore agents to a known-good state.

[License: MIT](LICENSE)

---

## Why Agent Doctor?

Developers often run **several** local agents at once:


| Runtime                                                       | Typical config              |
| ------------------------------------------------------------- | --------------------------- |
| [OpenClaw](https://github.com/openclaw/openclaw)              | `~/.openclaw/openclaw.json` |
| [Hermes Agent](https://github.com/nousresearch/hermes-agent)  | `~/.hermes/config.yaml`     |
| [Claude Code](https://docs.anthropic.com/en/docs/claude-code) | `~/.claude/settings.json`   |
| Codex CLI                                                     | `~/.codex/config.toml`      |


Each tool has its own install path, gateway settings, skills manifest, and failure modes. Agent Doctor gives you **one** place to answer:

- What is installed on this laptop?
- Where do configs live?
- Are runtimes pointed at the right gateway?
- Why did this agent stop working?
- What needs to be backed up before repair?
- Can we verify or restore a team profile before agents run?

```text
  Your laptop
 ┌─────────────────────────┐
 │ Agent Doctor            │
 │ doctor · repair · setup │
 └───────────┬─────────────┘
             │
   OpenClaw · Hermes · Claude Code · Codex
```

Optional: teams can plug in an enterprise control plane (e.g. [Evotown](https://github.com/EXboys/evotown)) for gateway keys, SkillHub, and policy — see [docs/enterprise.md](docs/enterprise.md).

---

## Status

🚧 **Early MVP** — Rust workspace + `agent-doctor doctor` + Tauri menubar shell. See [docs/ROADMAP.md](docs/ROADMAP.md) for remaining P0 items (`repair`, `setup`, `sync`, `policy pull`).

---

## Planned commands

```bash
# Discover installed runtimes, config paths, gateway wiring
agent-doctor doctor

# Back up, diagnose, and repair a broken runtime
agent-doctor repair openclaw

# Apply company profile (URL + API key + per-runtime config)
agent-doctor setup --url https://gateway.company.internal --key ...

# Pull private skill bundle from control plane
agent-doctor sync

# Cache policies from control plane
agent-doctor policy pull
```

---

## Relationship to other tools


| Project                                                     | Scope                                                                         |
| ----------------------------------------------------------- | ----------------------------------------------------------------------------- |
| **[ClawPanel](https://github.com/qingchencloud/clawpanel)** | Rich GUI for OpenClaw + Hermes                                                |
| **[ClawPal](https://github.com/lay2dev/clawpal)**           | OpenClaw desktop config companion                                             |
| **Agent Doctor**                                             | **Cross-runtime diagnosis, backup, repair, and team profile verification** |


---

## 中文

**Agent Doctor（本机 Agent 诊断与修复工具）** 用于在同一台电脑上**发现**已安装的 OpenClaw、Hermes、Claude Code、Codex 等，**备份配置、诊断故障、修复运行时**，并验证团队网关/Skill 配置模板。

详见 [docs/zh-CN/README.md](docs/zh-CN/README.md)。企业控制面集成（可选）见 [docs/enterprise.md](docs/enterprise.md)。

---

## Development

```bash
# CLI
cargo run -p agent-doctor -- doctor

# Local CI checks (fmt / clippy / test)
make check
# or: ./scripts/check.sh cli

# Desktop menubar (requires Node.js)
cd desktop && npm install && npm run tauri dev
```

See [docs/development.md](docs/development.md), [docs/ROADMAP.md](docs/ROADMAP.md), [docs/install.md](docs/install.md), [cli/README.md](cli/README.md), [desktop/README.md](desktop/README.md), and [CONTRIBUTING.md](CONTRIBUTING.md).

## Install

Prebuilt CLI and desktop bundles are published to [GitHub Releases](https://github.com/EXboys/agent-doctor/releases).

```bash
# Latest CLI (pick the pattern for your OS — see docs/install.md)
gh release download --repo EXboys/agent-doctor --pattern 'agent-doctor-*-macos-arm64.tar.gz'
tar -xzf agent-doctor-*-macos-arm64.tar.gz && chmod +x agent-doctor
./agent-doctor doctor
```

## License

MIT — see [LICENSE](LICENSE).