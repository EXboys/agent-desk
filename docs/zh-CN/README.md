# Agent Desk 中文说明

**Agent Desk（本机 Agent 工作台）** 是在员工笔记本电脑上统一管理多种桌面 Agent 的开源客户端。

## 解决什么问题

同一个人可能同时安装：

- **OpenClaw** — 常驻助手、Skill、派活  
- **Hermes** — 团队推送的另一套 Agent 运行时  
- **Claude Code** — IDE/终端里的 coding agent  
- **Codex CLI** — OpenAI coding agent  

各自配置文件路径不同，很难一眼看清：装了哪些、配置在哪、是否指向正确的公司网关。

Agent Desk 提供：

1. **发现** — 装了哪些、版本、配置在哪  
2. **配置** — 一键写入团队 profile 并合并各 Runtime 配置  
3. **验证** — `doctor` 检查网关与安装状态  
4. **同步** — 从控制面拉 Skill bundle（计划）

## 和 ClawPanel 的区别

- [ClawPanel](https://github.com/qingchencloud/clawpanel) 侧重 **OpenClaw + Hermes** 图形化管理。  
- Agent Desk 侧重 **跨 Runtime 本机发现与配置**，CLI 优先，桌面菜单栏作为轻量补充。

## 企业控制面（可选）

若团队部署了企业网关 / Skill 市场 / 策略服务，可通过 `setup` / `sync` / `policy pull` 对接。示例见 [enterprise.md](../enterprise.md)（含 [Evotown](https://github.com/EXboys/evotown) 集成说明）。

## 当前状态

🚧 **早期 MVP** — 已搭建 Rust workspace、`agent-desk doctor` 与 Tauri 菜单栏。`setup` / `sync` / `policy pull` 见 [ROADMAP.md](../ROADMAP.md)。

## 计划命令

```bash
agent-desk doctor
agent-desk setup --url https://gateway.company.internal --key ...
agent-desk sync
agent-desk policy pull
```

详见 [ROADMAP.md](../ROADMAP.md)。
