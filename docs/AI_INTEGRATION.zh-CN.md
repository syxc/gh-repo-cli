# AI 助手集成指南

> 让助手自己决定何时使用 ghr，而不是手动调用。

ghr 用 Git 克隆公开仓库并在本地分析，不使用 GitHub API 或 MCP 配额，克隆和刷新仍需要网络。下文以 Claude Code 为例，同样的规则也可加入 Codex 或 Pi 的指令文件。

## 配置

### 1. 安装 ghr

见 [README 安装一节](../README.zh-CN.md#安装)，然后确认：

```bash
which ghr && ghr --version
```

### 2. 在 Agent 的指令文件中添加规则

Claude Code 编辑 `~/.claude/CLAUDE.md`；Codex 等其他 Agent 读取各自的指令文件（如 `AGENTS.md`）。需要添加的规则：

```
# GitHub 仓库分析优先级

读取公开仓库文件时优先使用 ghr；它通过 Git 工作，不使用 GitHub API 或 MCP 配额。

触发规则（用户输入匹配 → 使用 ghr）:
- github.com/ 链接
- "github 仓库" | "分析.*仓库" | "仓库.*分析"
- "查看.*代码" | "阅读.*源码" | "clone.*github"
- owner/repo 格式（如 facebook/react）

使用方式：
ghr analyze <owner/repo>           # 完整分析
ghr search <owner/repo> <query>    # 搜索代码
ghr structure <owner/repo>         # 获取结构
ghr read <owner/repo> <file>       # 读取文件
ghr readme <owner/repo>            # 读取 README

MCP 备用条件（仅在以下情况使用 MCP）:
1. 私有仓库（ghr 仅支持公开仓库）
2. 需要 git 历史
3. 用户明确要求使用 MCP
```

### 3. 开始使用

```
你: "我想理解 Next.js 的架构"
Claude Code: [自动执行 ghr analyze vercel/next.js，然后回答]
```

## 使用模式

由面到点：先用 `ghr structure` 看结构，再用 `ghr search` 定位，最后用 `ghr read` 精读。例如：

```
你: "TypeScript 项目是怎么组织的？"
→ ghr structure microsoft/TypeScript --depth 2

你: "编译器入口在哪？"
→ ghr search microsoft/TypeScript "createCompiler" -e .ts

你: "给我看核心循环的代码"
→ ghr read microsoft/TypeScript src/compiler/tsc.ts
```

第一条命令克隆仓库，后续命令从当前默认分支刷新缓存；`--no-cache` 强制重新克隆。共用缓存的命令串行执行，避免并行调用同一缓存。需要结构化结果时用 `-o file.json`，例如 `@react-analysis.json 解释这个代码库的架构`。

## 故障排查

- **助手没有使用 ghr**：确认 `~/.claude/CLAUDE.md` 中存在优先级段落，并新开会话让其加载。
- **找不到 `ghr` 命令**：把 `~/.cargo/bin` 加入 `PATH`，或按 README 重新安装。
- **助手使用了 MCP 而不是 ghr**：私有仓库、需要 git 历史、或你明确要求 MCP 时属预期行为——ghr 仅支持公开仓库。
