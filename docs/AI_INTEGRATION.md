# Claude Code Integration

> Best practices for using gh-repo-cli with Claude Code

## Core Philosophy

**Let AI decide when to use tools, not manual invocation.**

By adding gh-repo-cli usage rules to your `~/.claude/CLAUDE.md` global configuration file, Claude Code will automatically detect when GitHub repository analysis is needed.

---

## Quick Setup

### 1. Install gh-repo-cli

```bash
git clone https://github.com/syxc/gh-repo-cli.git
cd gh-repo-cli
npm install
npm link
```

### 2. Add Configuration to CLAUDE.md

Edit `~/.claude/CLAUDE.md` file and add:

```
# GitHub Repository Analysis Priority

GLM MCP (zread): Limited quota ❌
gh-repo-cli: Completely free ✅

Trigger rules (user input matches → use gh-repo-cli):
- github.com/ links
- "github repository" | "analyze.*repository" | "repository.*analysis"
- "view.*code" | "read.*source" | "clone.*github"
- owner/repo format (e.g., facebook/react)

Usage:
ghr analyze <owner/repo>           # Full analysis
ghr search <owner/repo> <query>    # Search code
ghr structure <owner/repo>         # Get structure
ghr read <owner/repo> <file>       # Read file
ghr readme <owner/repo>            # Read README

MCP fallback conditions (only use zread MCP when):
1. Private repository (gh-repo-cli only supports public repositories)
2. Git history needed
3. User explicitly requests MCP
```

### 3. Start Using

```
You: "I want to understand Next.js architecture"
Claude Code: [Automatically runs ghr analyze vercel/next.js and analyzes]
```

---

## Comparative Advantages

| Aspect | MCP Server | gh-repo-cli |
|--------|-----------|-------------|
| **Usage Quota** | 100-500 times/month ❌ | Unlimited ✅ |
| **Rate Limit** | 60 requests/hour ❌ | No limit ✅ |
| **Cost** | $10-50/month ❌ | Completely free ✅ |
| **Privacy** | Code sent to third party ❌ | Local analysis ✅ |
| **Reliability** | Server dependent ❌ | Works offline ✅ |

---

## Usage Examples

### Scenario 1: Explore Project Architecture

```
You: "How is TypeScript organized?"

Claude Code:
  $ ghr structure microsoft/TypeScript --depth 2

  TypeScript project structure:
  ├── src/          # Compiler core code
  ├── scripts/      # Build scripts
  └── tests/        # Test files
```

### Scenario 2: Find Feature Implementation

```
You: "How does Vite implement HMR?"

Claude Code:
  $ ghr search vitejs/vite "HMR|hot.*module"
  $ ghr read vitejs/vite packages/vite/src/server/moduleGraph.ts

  Vite's HMR is implemented in moduleGraph.ts, using WebSocket connections...
```

### Scenario 3: Learn Configuration Standards

```
You: "What configuration file formats does ESLint support?"

Claude Code:
  $ ghr search eslint/eslint "config.*file" -e .md
  $ ghr read eslint/eslint docs/use/configuration/README.md

  ESLint supports .eslintrc.js, .eslintrc.json, .eslintrc.yml, etc...
```

### Scenario 4: Compare Project Differences

```
You: "What's the difference between Webpack and Vite's build approach?"

Claude Code:
  $ ghr analyze webpack/webpack
  $ ghr analyze vitejs/vite

  Key differences:
  - Webpack uses bundling, Vite uses ESM + native browser
  - Vite has faster startup and better dev experience...
```

---

## Best Practices

### Ask Naturally, Let AI Lead

✅ **Correct**: "How is Tailwind CSS structured?" → Claude automatically calls ghr  
❌ **Wrong**: "Run ghr analyze tailwindlabs/tailwindcss"

### Broad to Deep, Progressive Approach

```
Recommended workflow:
  1. "What's the project structure?" → ghr structure
  2. "Where is the routing implemented?" → ghr search
  3. "Show me the router core code" → ghr read
```

### Leverage Caching

```bash
# First analysis: Clone repository (10-30 seconds)
ghr analyze vercel/next.js

# Subsequent queries: Use cache (<1 second)
ghr search vercel/next.js "router"
ghr read vercel/next.js packages/next/server/router.ts

# Force refresh when needed
ghr analyze vercel/next.js --no-cache
```

---

## Troubleshooting

### Claude Isn't Using gh-repo-cli

**Check configuration**:
```bash
cat ~/.claude/CLAUDE.md
```

Should contain the "GitHub Repository Analysis Priority" section

### ghr Command Not Found

**Reinstall**:
```bash
git clone https://github.com/syxc/gh-repo-cli.git
cd gh-repo-cli
npm install
npm link
```

**Verify installation**:
```bash
which ghr
# Should output: /usr/local/bin/ghr
```

### Claude Uses MCP Instead of gh-repo-cli

This is **expected behavior**! Automatically falls back to MCP when:
1. Private repository
2. Git history needed
3. User explicitly requests MCP

---

## Summary

**Configure once, use forever**:
1. Install gh-repo-cli (1 minute)
2. Add configuration to CLAUDE.md (10 seconds)
3. Natural conversation to use (zero learning)

**Key advantages**:
- ✅ AI auto-detection, no manual invocation needed
- ✅ Unlimited usage, completely free
- ✅ Local analysis, privacy secure
- ✅ Smart fallback, supports private repos

---

<div align="center">

**CLAUDE.md instructions + gh-repo-cli = Automatic repository analysis** 🚀

</div>
