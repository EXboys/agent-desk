# Agent Desk 中文说明

**Agent Desk（本机 Agent 工作台）** 是在员工笔记本电脑上统一管理多种桌面 Agent 的开源客户端。

## 解决什么问题

同一个人可能同时安装：

- **OpenClaw** — 常驻助手、Skill、派活  
- **Hermes** — 团队推送的另一套 Agent 运行时  
- **Claude Code** — IDE/终端里的 coding agent  
- **Codex CLI** — OpenAI coding agent  

各自配置文件路径不同，IT 难以确认是否都接入公司 **Evotown** 网关（`evk_`）、是否从私有 SkillHub 拉包。

Agent Desk 提供：

1. **发现** — 装了哪些、版本、配置在哪  
2. **配置** — 一键写入 `evotown.agent.env` 并合并各 Runtime 配置  
3. **验证** — `doctor` 检查是否指向公司网关  
4. **同步** — 从 Evotown 拉 Skill bundle（计划）

公司侧监控、策略、审计仍在 **[Evotown](https://github.com/EXboys/evotown)** 控制面完成。

## 和 ClawPanel 的区别

- [ClawPanel](https://github.com/qingchencloud/clawpanel) 侧重 **OpenClaw + Hermes** 图形化管理。  
- Agent Desk 侧重 **跨 Runtime + Evotown 企业模板**，CLI 优先，桌面托盘后续补充。

## 当前状态

仓库已创建，CLI 开发中。过渡期请使用 Evotown 仓库内脚本：

```bash
python3 scripts/evotown-agent-setup.py check
python3 scripts/evotown-agent-setup.py sync
```

## 计划命令

```bash
agent-desk doctor
agent-desk setup --url https://evotown.company.internal --key evk_xxxx
agent-desk sync
agent-desk policy pull
```

详见 [ROADMAP.md](../ROADMAP.md)。
