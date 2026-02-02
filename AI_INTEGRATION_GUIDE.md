# AI Integration Guide for gh-repo-cli

> Complete guide on integrating gh-repo-cli with AI coding assistants using the **CLAUDE.md global instruction** approach

## 📚 Table of Contents

- [Why This Approach is Better](#why-this-approach-is-better)
- [Claude Code Integration (Recommended)](#claude-code-integration-recommended)
- [Other AI Assistants](#other-ai-assistants)
- [Advanced Workflows](#advanced-workflows)
- [Best Practices](#best-practices)

---

## Why This Approach is Better

### The Problem with MCP Servers

Many AI coding assistants provide GitHub repository analysis through MCP (Model Context Protocol) servers. However, these have significant limitations:

| Issue | MCP Servers | gh-repo-cli |
|-------|-------------|-------------|
| **Usage Quotas** | 100-500 requests/month ❌ | Unlimited ✅ |
| **Rate Limits** | 60 requests/hour ❌ | No API calls ✅ |
| **Cost** | $10-50/month ❌ | Completely free ✅ |
| **Privacy** | Code sent to third-party ❌ | Local analysis ✅ |
| **Reliability** | Server downtime ❌ | Works offline ✅ |

### The Key Insight: Let AI Decide

**Best Practice**: Don't create complex skill scripts. Instead, **write simple instructions in your CLAUDE.md global configuration file** and let AI decide when to use gh-repo-cli.

**Why this works better**:
- ✅ AI actively detects when you need repository analysis
- ✅ No need to manually invoke skills
- ✅ Automatic fallback to MCP when needed
- ✅ Simple one-time configuration
- ✅ Works for all conversations, not just specific prompts

---

## Claude Code Integration (Recommended)

### The One-Time Configuration

Add this section to your **`~/.claude/CLAUDE.md`** global instruction file:

```markdown
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
```

### How It Works

Once configured, Claude Code will **automatically detect** when you need repository analysis:

#### Example 1: Implicit Detection

```
You: How does React implement hooks?

Claude Code:
I'll analyze the React repository to understand the hooks implementation.

$ ghr analyze facebook/react
[Analyzing repository structure...]

$ ghr search facebook/react "useState" -e .js
[Searching for useState implementations...]

$ ghr read facebook/react packages/react/src/ReactHooks.js
[Reading key file...]

Based on my analysis of the React repository, here's how hooks are implemented...
```

#### Example 2: Direct Request

```
You: Analyze the Vue.js repository structure

Claude Code:
$ ghr structure vuejs/core --depth 3
[Getting directory tree...]

Here's the Vue.js core repository structure:
- packages/reactivity: ...
- packages/compiler-core: ...
- packages/runtime-dom: ...
```

#### Example 3: owner/repo Format

```
You: Compare facebook/react and vuejs/core

Claude Code:
$ ghr analyze facebook/react
$ ghr analyze vuejs/core

Let me compare these two frameworks...
```

### Why This Approach is Superior

| Aspect | Skill-Based Approach | CLAUDE.md Approach |
|--------|---------------------|-------------------|
| **Setup Complexity** | Create JSON skill files | One markdown snippet |
| **AI Awareness** | Must explicitly invoke | Automatically detects |
| **Flexibility** | Fixed command templates | AI adapts to context |
| **Maintenance** | Update skill files | Never needs updates |
| **User Intent** | "Use the github-repo-analyzer skill" | Natural: "How does React work?" |

### Where to Add This Configuration

The configuration should be added to your **global** CLAUDE.md file at:

```bash
~/.claude/CLAUDE.md
```

**Important**: This is your personal global configuration, NOT the CLAUDE.md file in any project directory. Adding it to project directories would cause those projects to automatically load gh-repo-cli instructions for all users.

**How to add**:

1. Open or create `~/.claude/CLAUDE.md`
2. Append the "GitHub 仓库分析优先级" section (shown above)
3. Save the file

That's it! Claude Code will now automatically use gh-repo-cli when needed.

### Automatic Fallback to MCP

The instruction also specifies when to fall back to MCP servers:

```
MCP 备用条件:
1. 私有仓库（gh-repo-cli 仅支持公开仓库）
2. 需要 git 历史
3. 用户明确要求使用 MCP
```

**Example**:

```
You: Analyze my-private-org/private-repo

Claude Code:
I detect this is a private repository. gh-repo-cli only supports public repositories,
so I'll use the MCP server instead.

[Uses zread MCP to access private repository]
```

---

## Other AI Assistants

### Cursor / Windsurf / GitHub Copilot

These tools have built-in terminal integration. The workflow is:

```bash
# Run gh-repo-cli in the terminal
ghr analyze facebook/react -o react.json
ghr search facebook/react "useState" -o hooks.json

# Reference the output in the AI chat
@react.json @hooks.json
Explain how React implements useState based on this analysis
```

**Tip**: You can also add the CLAUDE.md instructions to these tools if they support global instructions (check tool settings).

### ChatGPT / Claude (Web)

Since web-based AI tools can't run commands directly:

```bash
# 1. Export repository data
ghr analyze facebook/react -o react.json
ghr search facebook/react "fiber" -e .js -o fiber.json

# 2. Upload JSON files to ChatGPT/Claude

# 3. Ask questions with context
"I'm studying React's Fiber architecture. Here's the repository analysis
and search results for 'fiber'. Can you explain how the reconciler works?"
```

---

## Advanced Workflows

### Workflow 1: Comparative Analysis

Compare two frameworks/libraries:

```
You: What are the key differences between React and Vue's reactivity systems?

Claude Code:
$ ghr analyze facebook/react
$ ghr search facebook/react "useState" -e .js

$ ghr analyze vuejs/core
$ ghr search vuejs/core "reactive" -e .ts

[Analyzing both implementations...]

Key differences between React and Vue reactivity:

1. **State Management**:
   - React: Manual updates via setState/useState
   - Vue: Automatic tracking via Proxies

2. **Re-rendering**:
   - React: Explicit dependency arrays in useEffect
   - Vue: Implicit dependency tracking

3. **Performance**:
   - React: Requires memo/useMemo for optimization
   - Vue: Automatically skips unchanged dependencies
```

### Workflow 2: Migration Planning

Plan migration from one library to another:

```
You: We want to migrate from Moment.js to date-fns. What should we consider?

Claude Code:
$ ghr search your-org/your-repo "moment" -o current-usage.json
$ ghr analyze moment/moment -o moment-api.json
$ ghr analyze date-fns/date-fns -o datefns-api.json

[Analyzing API differences...]

Migration Plan: Moment.js → date-fns

1. **Breaking Changes**:
   - moment() is mutable → date-fns is immutable
   - Formatting tokens differ (YYYY vs yyyy)

2. **API Mapping**:
   - moment.add() → add()
   - moment.format() → format()
   - moment.diff() → differenceIn()

3. **Recommended Approach**:
   - Step 1: Install date-fns
   - Step 2: Replace simple cases first
   - Step 3: Handle complex parsing logic
   - Step 4: Remove moment.js dependency
```

### Workflow 3: Bug Investigation

Investigate issues in open-source projects:

```
You: I'm experiencing a useEffect cleanup bug. Can you help me understand how it should work?

Claude Code:
$ ghr analyze facebook/react
$ ghr search facebook/react "useEffect.*cleanup" -e .js
$ ghr read facebook/react packages/react/src/ReactHooks.js
$ ghr read facebook/react packages/react/src/ReactFiberCommitWork.js

[Reading implementation...]

Here's how useEffect cleanup works in React:

1. **Registration**: When useEffect is called, React registers the effect
2. **Execution**: Effects run after the paint phase
3. **Cleanup**: Cleanup functions run before the next effect executes

Common bug causes:
- ❌ Forgetting to return cleanup function
- ❌ Cleanup function throws an error
- ❌ Missing dependency array
```

---

## Best Practices

### 1. Let AI Lead

**Don't**: Manually decide which tool to use

**Do**: Trust the instructions in CLAUDE.md

```
❌ "I should use ghr for this. Let me run ghr analyze..."
✅ "How does React work?" → Claude automatically runs ghr analyze
```

### 2. Use Natural Language

**Don't**: Use technical commands

**Do**: Ask questions naturally

```
❌ "Run ghr analyze facebook/react"
✅ "How is React structured?"
```

### 3. Leverage Cache

gh-repo-cli caches repositories in `~/.ghr-cache/`:

```bash
# First run: clones the repo
ghr analyze facebook/react  # Takes 10-30 seconds

# Subsequent runs: uses cache (instant)
ghr analyze facebook/react  # Takes <1 second

# Force refresh if needed
ghr analyze facebook/react --no-cache
```

### 4. Start Broad, Then Deep

```
Good workflow:
1. "What is the Vue.js repository structure?"
   → ghr structure vuejs/core

2. "How does reactivity work?"
   → ghr search vuejs/core "reactive"

3. "Show me the reactive implementation"
   → ghr read vuejs/core packages/reactivity/reactive.ts
```

### 5. Respect Limits

For very large repositories (monorepos like Microsoft/vscode):

```bash
# AI will automatically:
- Limit structure depth (--depth 2)
- Search specific subdirectories
- Read only key files
- Avoid searching entire codebase
```

### 6. Combine with Other Tools

The CLAUDE.md approach makes it easy to combine tools:

```
You: What are the latest trends in state management and how does Redux compare?

Claude Code:
# Uses web search for trends
$ npx ducksearch search "state management trends 2024"

# Uses gh-repo-cli for Redux analysis
$ ghr analyze reduxjs/redux
$ ghr search reduxjs/redux "createStore"

Combining latest trends with implementation details...
```

---

## Troubleshooting

### Issue: Claude doesn't use gh-repo-cli

**Cause**: Instructions not added to CLAUDE.md or wrong file location

**Solution**:
```bash
# Verify file exists
cat ~/.claude/CLAUDE.md

# Should contain the GitHub 仓库分析优先级 section
```

### Issue: ghr command not found

**Cause**: gh-repo-cli not installed or not in PATH

**Solution**:
```bash
# Install globally
npm install -g gh-repo-cli

# Or use npm link from source
cd /path/to/gh-repo-cli
npm link

# Verify installation
which ghr
```

### Issue: Claude uses MCP instead of gh-repo-cli

**Cause**: Repository is private or AI needs git history

**This is expected behavior**! The instructions specify:
1. Private repos → Use MCP
2. Need git history → Use MCP
3. User explicitly requests MCP → Use MCP

### Issue: Repository too large to clone

**Workaround**: Focus on specific directories

```bash
# AI will automatically:
ghr structure facebook/react --depth 2  # Shallow tree
ghr search facebook/react "useState" -d packages/react  # Specific dir
ghr read facebook/react README.md  # Single file
```

---

## Conclusion

The **CLAUDE.md global instruction approach** is the best way to integrate gh-repo-cli with AI assistants because:

1. **Zero Friction**: One-time configuration, automatic detection
2. **Natural Interaction**: Ask questions naturally, AI decides tools
3. **Smart Fallback**: Automatically uses MCP when gh-repo-cli can't help
4. **Always Active**: Works for all conversations, not just specific prompts
5. **Easy Maintenance**: No skill files to update or maintain

### Quick Start

1. **Install gh-repo-cli**:
   ```bash
   npm install -g gh-repo-cli
   ```

2. **Add instructions to CLAUDE.md**:
   ```bash
   # Edit ~/.claude/CLAUDE.md
   # Add the "GitHub 仓库分析优先级" section (see above)
   ```

3. **Start asking questions**:
   ```
   "How does Next.js routing work?"
   "What's the architecture of TypeScript?"
   "Compare React and Vue reactivity"
   ```

4. **Claude automatically uses gh-repo-cli** when needed!

---

## Example: Complete Conversation

Here's how a natural conversation flows:

```
You: How does React Server Components work?

Claude: Let me analyze the React repository to understand Server Components.

$ ghr analyze facebook/react
✓ Repository: facebook/react
✓ Stars: 220k
✓ Main language: JavaScript

$ ghr search facebook/react "Server Components" -i
Found 47 files matching "Server Components"

$ ghr read facebook/react README.md
[Reading README...]

$ ghr search facebook/react "createServerComponent" -e .js -e .ts
Found 12 files

$ ghr read facebook/react packages/react-server-dom/src/ReactFlightServer.js
[Reading Server Components implementation...]

Based on my analysis of the React repository, here's how Server Components work:

## Architecture

Server Components run only on the server and serialize their output to the client:

1. **Server Execution**:
   - Components run in Node.js environment
   - Can access databases, filesystem, internal APIs
   - Zero bundle size sent to client

2. **Serialization**:
   - React serializes the component tree to JSON
   - Client-side React hydrates from the JSON
   - Boundary between Server and Client Components

3. **Key Implementation**:
   The core is in `ReactFlightServer.js`, which handles:
   - Component traversal
   - Promise resolution for async data
   - Serialization of virtual DOM to JSON

[... continues with detailed explanation ...]

You: Can you show me the code that handles the serialization?

Claude: $ ghr read facebook/react packages/react-server-dom/src/ReactFlightServer.js
[Shows full implementation...]

You: What about the client-side deserialization?

Claude: $ ghr read facebook/react packages/react-server-dom/src/ReactFlightClient.js
[Shows client-side hydration code...]
```

**Key Point**: You just asked natural questions. Claude automatically decided when to run ghr commands!

---

<div align="center">

**Happy coding! 🚀**

**CLAUDE.md instructions + gh-repo-cli = Automatic repository analysis**

</div>
