# Agent Doctor

**Diagnose, repair, and onboard local AI agent runtimes on one machine.**

Agent Doctor discovers OpenClaw, Hermes, Claude Code, Codex, and related runtimes, runs redacted probes to find misconfiguration, and repairs them with backups, typed actions, and audit reports.

Use it standalone on a developer laptop, or connect an enterprise control plane when your team needs shared gateways, skills, and policy.

```bash
agent-doctor doctor                    # Diagnose: installed runtimes, config paths, gateway wiring
agent-doctor repair hermes             # Repair: deep checks + safe repair preview (execution rolling out)
agent-doctor setup --url ... --key ... # Onboard: apply company profile to runtimes (planned)
```

[License: MIT](LICENSE) · [Roadmap](docs/ROADMAP.md) · [Repair safety](docs/repair-safety.md)

---

## Enterprise (optional)

With a control plane (e.g. [Evotown](https://github.com/EXboys/evotown)):

```bash
agent-doctor sync          # Pull private skill bundle (planned)
agent-doctor policy pull   # Cache policy rules locally (planned)
```

See [docs/enterprise.md](docs/enterprise.md).

---

## Status

🚧 **Early MVP** — `agent-doctor doctor`, read-only `repair` probes, and a Tauri menubar shell. P0 next: `repair` execution and `setup`. See [docs/ROADMAP.md](docs/ROADMAP.md).

Diagnostic data is classified by sensitivity; secrets are redacted before AI analysis. Real writes require typed actions, backups, and confirmation — [docs/repair-safety.md](docs/repair-safety.md).

---

## Why Agent Doctor?

Developers and teams increasingly run **several** local AI agent runtimes:


| Runtime                                                       | Typical config              |
| ------------------------------------------------------------- | --------------------------- |
| [OpenClaw](https://github.com/openclaw/openclaw)              | `~/.openclaw/openclaw.json` |
| [Hermes Agent](https://github.com/nousresearch/hermes-agent)  | `~/.hermes/config.yaml`     |
| [Claude Code](https://docs.anthropic.com/en/docs/claude-code) | `~/.claude/settings.json`   |
| Codex CLI                                                     | `~/.codex/config.toml`      |


Each runtime has its own install path, gateway settings, skills manifest, policy surface, and failure modes. Agent Doctor gives you **one** local client to answer:

- What is installed on this laptop?
- Where do configs live?
- Are runtimes pointed at the approved company gateway?
- Which configs drifted away from the team profile?
- Why did this employee's agent stop working?
- What needs to be backed up before repair or policy remediation?
- Can we safely restore the runtime to a compliant team baseline?

```text
  Your laptop
 ┌─────────────────────────┐
 │ Agent Doctor            │
 │ doctor · repair · setup │
 └───────────┬─────────────┘
             │
   OpenClaw · Hermes · Claude Code · Codex
```

---

## Relationship to other tools


| Project                                                     | Scope                                                                         |
| ----------------------------------------------------------- | ----------------------------------------------------------------------------- |
| **[ClawPanel](https://github.com/qingchencloud/clawpanel)** | Rich GUI for OpenClaw + Hermes                                                |
| **[ClawPal](https://github.com/lay2dev/clawpal)**           | OpenClaw desktop config companion                                             |
| **Agent Doctor**                                             | **Team runtime diagnosis, backup, repair, policy checks, and compliance reporting** |


---

## 中文

**Agent Doctor** 在本机 **诊断、修复、就位** AI Agent Runtime（OpenClaw、Hermes、Claude Code、Codex 等）。

```bash
agent-doctor doctor                    # 诊断
agent-doctor repair hermes             # 修复（深度检查 + 安全预览，执行能力逐步开放）
agent-doctor setup --url ... --key ... # 就位（规划中）
```

企业可选：`sync`、`policy pull` — 见 [docs/enterprise.md](docs/enterprise.md)。完整中文说明：[docs/zh-CN/README.md](docs/zh-CN/README.md)。

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
