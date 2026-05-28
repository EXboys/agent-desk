# Agent Doctor 中文说明

**Agent Doctor（本机 Agent 诊断与修复工具）** 是在员工笔记本电脑上诊断、备份并修复多种桌面 Agent runtime 的开源客户端。

## 解决什么问题

同一个人可能同时安装：

- **OpenClaw** — 常驻助手、Skill、派活  
- **Hermes** — 团队推送的另一套 Agent 运行时  
- **Claude Code** — IDE/终端里的 coding agent  
- **Codex CLI** — OpenAI coding agent  

各自安装路径、配置文件、网关、Skill/MCP 配置和日志位置都不同。Agent 出问题时，很难快速判断是安装损坏、配置漂移、环境变量冲突，还是团队网关配置错误。

Agent Doctor 提供：

1. **发现** — 装了哪些、版本、配置在哪  
2. **诊断** — `doctor` 检查安装、配置、网关与密钥来源  
3. **备份** — 修复前保存 runtime 配置快照（计划）  
4. **修复** — 针对 OpenClaw、Hermes、Claude Code、Codex 等生成并执行修复方案（计划）  
5. **同步** — 从控制面拉团队 profile、Skill bundle 和 policy（计划）

## 和 ClawPanel 的区别

- [ClawPanel](https://github.com/qingchencloud/clawpanel) 侧重 **OpenClaw + Hermes** 图形化管理。  
- Agent Doctor 侧重 **跨 Runtime 本机诊断、备份、修复与团队配置验证**，CLI 优先，桌面菜单栏作为轻量补充。

## 企业控制面（可选）

若团队部署了企业网关 / Skill 市场 / 策略服务，可通过 `setup` / `sync` / `policy pull` 对接。示例见 [enterprise.md](../enterprise.md)（含 [Evotown](https://github.com/EXboys/evotown) 集成说明）。

## 当前状态

🚧 **早期 MVP** — 已搭建 Rust workspace、`agent-doctor doctor` 与 Tauri 菜单栏。`repair` / `setup` / `sync` / `policy pull` 见 [ROADMAP.md](../ROADMAP.md)。

## 计划命令

```bash
agent-doctor doctor
agent-doctor repair openclaw
agent-doctor setup --url https://gateway.company.internal --key ...
agent-doctor sync
agent-doctor policy pull
```

详见 [ROADMAP.md](../ROADMAP.md)。
