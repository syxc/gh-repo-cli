# CLAUDE.md Snippet for gh-repo-cli

> Copy this section and add it to your `~/.claude/CLAUDE.md` global instruction file

---

# GitHub 仓库分析优先级

```
GLM MCP (zread): 有限配额 ❌
gh-repo-cli: 完全免费 ✅
```

```
触发规则（用户输入匹配 → 使用 gh-repo-cli）:
github\.com/|github 仓库|分析.*仓库|仓库.*分析
查看.*代码|阅读.*源码|clone.*github
owner/repo 格式（如 facebook/react）
```

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

---

## How to Use

1. **Open your CLAUDE.md file**:
   ```bash
   cat ~/.claude/CLAUDE.md
   # or create it if it doesn't exist
   touch ~/.claude/CLAUDE.md
   ```

2. **Copy the snippet above** and paste it into your CLAUDE.md file

3. **Restart Claude Code** (reload global instructions)

4. **Start asking questions naturally**:
   ```
   "How does React work?"
   "Analyze the Vue.js repository"
   "Compare Redux and MobX"
   ```

5. **Claude will automatically use gh-repo-cli** when it detects you need repository analysis!

---

## Complete CLAUDE.md Example

Here's a complete `~/.claude/CLAUDE.md` file showing where to add the configuration:

```markdown
# Role and Goal
You are a Senior Software Engineer. Your primary goal is to produce code that is clean, maintainable, secure, and robust.

# Core Principles
1. **Simplicity & Practicality (KISS & YAGNI)**: Prioritize clear, straightforward solutions
2. **Defensive Programming & Security**: Never trust external input
3. **Robustness & Fault Tolerance**: Anticipate failures and handle errors gracefully
4. **Maintainability (DRY)**: Don't repeat yourself
5. **Robust Design (SOLID)**: High cohesion, low coupling

# Output Format
- **Language**: All explanations in Chinese
- **Code Comments**: All comments in English

---

# GitHub 仓库分析优先级

```
GLM MCP (zread): 有限配额 ❌
gh-repo-cli: 完全免费 ✅
```

```
触发规则（用户输入匹配 → 使用 gh-repo-cli）:
github\.com/|github 仓库|分析.*仓库|仓库.*分析
查看.*代码|阅读.*源码|clone.*github
owner/repo 格式（如 facebook/react）
```

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

---

# 网络搜索优先级

```
GLM MCP: 100 次/月 ❌
ducksearch: 无限 ✅
```

触发规则（用户输入匹配 → 使用 ducksearch）:
搜索|search|查找|查一下|搜一下|搜资料
获取网页|fetch|抓取|网页内容|打开网页
最新|recent|新闻|news|当前|最近

使用方式：
```bash
npx ducksearch search "关键词"
npx ducksearch fetch https://example.com
```

[... add your other custom instructions here ...]
```

---

## Why This Approach?

### ❌ Old Way (Skill-Based)

```json
// ~/.claude/skills/github-repo-analyzer.json
{
  "name": "github-repo-analyzer",
  "command": "ghr",
  "arguments": {...}
}
```

**Problems**:
- Complex JSON configuration
- Must explicitly invoke: "Use the github-repo-analyzer skill"
- Fixed command templates
- Requires maintenance

### ✅ New Way (CLAUDE.md Instructions)

Just add a simple markdown snippet to your global instructions!

**Benefits**:
- ✅ Simple markdown (no JSON)
- ✅ Natural language: "How does React work?"
- ✅ AI adapts to context
- ✅ Automatic fallback to MCP
- ✅ Set once, never update

---

## Example Conversations

### Example 1: Natural Question

```
You: How does React Server Components work?

Claude: Let me analyze the React repository to understand Server Components.

$ ghr analyze facebook/react
✓ Repository: facebook/react
✓ Stars: 220k
✓ Main language: JavaScript

$ ghr search facebook/react "Server Components" -i
Found 47 files matching "Server Components"

$ ghr read facebook/react packages/react-server-dom/src/ReactFlightServer.js
[Reading implementation...]

Based on my analysis, here's how Server Components work...
```

### Example 2: Comparative Analysis

```
You: Compare React and Vue's reactivity systems

Claude: $ ghr analyze facebook/react
$ ghr search facebook/react "useState" -e .js

$ ghr analyze vuejs/core
$ ghr search vuejs/core "reactive" -e .ts

Key differences between React and Vue reactivity:
1. React: Manual updates via setState/useState
2. Vue: Automatic tracking via Proxies
[...]
```

### Example 3: Explicit Repository Format

```
You: What's the structure of facebook/react?

Claude: $ ghr structure facebook/react --depth 3

Here's the React repository structure:
- packages/react: Core React library
- packages/react-dom: DOM renderers
- packages/react-reconciler: Reconciliation algorithm
[...]
```

---

## Automatic Fallback

The instructions specify when to fall back to MCP servers:

```
MCP 备用条件:
1. 私有仓库（gh-repo-cli 仅支持公开仓库）
2. 需要 git 历史
3. 用户明确要求使用 MCP
```

**Example**:

```
You: Analyze my-private-org/private-repo

Claude: I detect this is a private repository. gh-repo-cli only supports
public repositories, so I'll use the MCP server instead.

[Uses zread MCP to access private repository]
```

---

## Verification

After adding the snippet, verify it works:

```bash
# 1. Check CLAUDE.md exists and contains the snippet
cat ~/.claude/CLAUDE.md | grep -A 20 "GitHub 仓库分析优先级"

# 2. Verify ghr is installed
which ghr

# 3. Test with Claude Code
# Ask: "How does React work?"
# Claude should automatically run ghr commands
```

---

## Troubleshooting

### Claude doesn't use gh-repo-cli

**Cause**: Instructions not added to CLAUDE.md

**Solution**:
```bash
# Verify file exists
cat ~/.claude/CLAUDE.md

# Should contain the GitHub 仓库分析优先级 section
```

### ghr command not found

**Cause**: gh-repo-cli not installed

**Solution**:
```bash
npm install -g gh-repo-cli
which ghr  # Should show /usr/local/bin/ghr or similar
```

### Claude uses MCP instead of gh-repo-cli

**This is expected** if:
- Repository is private
- You need git history
- You explicitly asked to use MCP

---

## See Also

- 📖 [AI_INTEGRATION_GUIDE.md](AI_INTEGRATION_GUIDE.md) - Comprehensive integration guide
- 📖 [README.md](README.md) - Project documentation
- 🚀 [Release Workflow](RELEASE_WORKFLOW.md) - How to create releases

---

<div align="center">

**Ready to supercharge your AI coding assistant?**

**1. Copy the snippet at the top of this file**
**2. Paste into ~/.claude/CLAUDE.md**
**3. Start asking questions naturally**

**That's it! 🎉**

</div>
