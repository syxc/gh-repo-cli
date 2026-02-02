# Claude Code Integration

> gh-repo-cli 与 Claude Code 结合使用的最佳实践

## 核心理念

**让 AI 主动判断何时使用工具，而不是手动调用。**

将 gh-repo-cli 的使用规则添加到你的 `~/.claude/CLAUDE.md` 全局配置文件中，Claude Code 会自动检测何时需要分析 GitHub 仓库。

---

## 一键配置

### 步骤 1: 安装 gh-repo-cli

```bash
npm install -g gh-repo-cli
```

### 步骤 2: 添加配置到 CLAUDE.md

编辑（或创建）`~/.claude/CLAUDE.md` 文件，添加以下内容：

```markdown
# GitHub 仓库分析优先级

```
GLM MCP (zread): 有限配额 ❌
gh-repo-cli: 完全免费 ✅
```

触发规则（用户输入匹配 → 使用 gh-repo-cli）:
- github.com/ 链接
- "github 仓库" | "分析.*仓库" | "仓库.*分析"
- "查看.*代码" | "阅读.*源码" | "clone.*github"
- owner/repo 格式（如 facebook/react）

使用方式：
```bash
ghr analyze <owner/repo>           # 完整分析
ghr search <owner/repo> <query>    # 搜索代码
ghr structure <owner/repo>         # 获取结构
ghr read <owner/repo> <file>       # 读取文件
ghr readme <owner/repo>            # 读取 README
```

MCP 备用条件（仅在以下情况使用 zread MCP）:
1. 私有仓库（gh-repo-cli 仅支持公开仓库）
2. 需要 git 历史
3. 用户明确要求使用 MCP
```

### 步骤 3: 开始使用

现在你可以直接用自然语言与 Claude Code 对话：

```
You: "React 是如何实现 hooks 的？"

Claude Code: [自动运行 ghr analyze facebook/react 并分析]
```

---

## 对比优势

| 方面 | MCP 服务器 | gh-repo-cli |
|------|-----------|-------------|
| **使用配额** | 100-500 次/月 ❌ | 无限 ✅ |
| **速率限制** | 60 请求/小时 ❌ | 无限制 ✅ |
| **费用** | $10-50/月 ❌ | 完全免费 ✅ |
| **隐私** | 代码发送到第三方 ❌ | 本地分析 ✅ |
| **可靠性** | 依赖服务器 ❌ | 离线工作 ✅ |

---

## 工作流程示例

### 示例 1: 隐式检测

```
You: "How does React implement hooks?"

Claude Code:
  $ ghr analyze facebook/react
  $ ghr search facebook/react "useState" -e .js
  $ ghr read facebook/react packages/react/src/ReactHooks.js

  Based on the repository analysis, here's how React implements hooks...
```

### 示例 2: 直接请求

```
You: "Analyze the Vue.js repository structure"

Claude Code:
  $ ghr structure vuejs/core --depth 3

  Here's the Vue.js repository structure...
```

### 示例 3: 仓库格式

```
You: "Compare facebook/react and vuejs/core"

Claude Code:
  $ ghr analyze facebook/react
  $ ghr analyze vuejs/core

  Key differences between React and Vue...
```

---

## 高级用法

### 比较分析

```
You: "React 和 Vue 的响应式系统有什么区别？"

Claude Code:
  $ ghr analyze facebook/react
  $ ghr search facebook/react "useState" -e .js
  $ ghr analyze vuejs/core
  $ ghr search vuejs/core "reactive" -e .ts

  比较分析结果...
```

### Bug 调查

```
You: "我遇到了 useEffect cleanup 的问题"

Claude Code:
  $ ghr analyze facebook/react
  $ ghr search facebook/react "useEffect.*cleanup" -e .js
  $ ghr read facebook/react packages/react/src/ReactHooks.js

  这里是 useEffect cleanup 的工作原理...
```

### 迁移规划

```
You: "我们要从 Moment.js 迁移到 date-fns"

Claude Code:
  $ ghr search your-org/your-repo "moment"
  $ ghr analyze moment/moment
  $ ghr analyze date-fns/date-fns

  迁移建议...
```

---

## 最佳实践

### 1. 让 AI 主导

✅ **正确**: "React 是如何工作的？" → Claude 自动运行 ghr
❌ **错误**: "运行 ghr analyze facebook/react"

### 2. 使用自然语言

✅ **正确**: "Vue 的结构是什么？"
❌ **错误**: "执行 ghr structure vuejs/core"

### 3. 利用缓存

```bash
# 首次运行：克隆仓库（10-30 秒）
ghr analyze facebook/react

# 后续运行：使用缓存（<1 秒）
ghr analyze facebook/react

# 强制刷新
ghr analyze facebook/react --no-cache
```

### 4. 先宽后深

```
Good workflow:
  1. "Vue.js 的仓库结构是什么？" → ghr structure
  2. "响应式是如何工作的？" → ghr search
  3. "展示响应式实现代码" → ghr read
```

---

## 故障排查

### Claude 没有使用 gh-repo-cli

**检查**:
```bash
cat ~/.claude/CLAUDE.md
```

应该包含 "GitHub 仓库分析优先级" 部分

### ghr 命令未找到

**安装**:
```bash
npm install -g gh-repo-cli
```

**验证**:
```bash
which ghr
```

### Claude 使用 MCP 而不是 gh-repo-cli

这是**预期行为**！当以下情况时会自动降级到 MCP：
1. 私有仓库
2. 需要 git 历史
3. 用户明确要求使用 MCP

---

## 总结

**配置一次，永久生效**：
1. 安装 gh-repo-cli（30 秒）
2. 添加配置到 CLAUDE.md（10 秒）
3. 开始自然对话（零学习）

**优势**：
- ✅ AI 自动检测，无需手动调用
- ✅ 自然语言交互
- ✅ 智能降级到 MCP（私有仓库时）
- ✅ 无限使用，完全免费

---

<div align="center">

**CLAUDE.md 指令 + gh-repo-cli = 自动仓库分析** 🚀

</div>
