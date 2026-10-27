# AI Assistant Integration

> Let the assistant decide when to use ghr, instead of invoking it by hand.

AI coding assistants analyze GitHub repositories through MCP servers or built-in API tools, which usually come with quotas. ghr is free and unlimited for public repositories, so it works well as the default repository-analysis tool. The same configuration pattern applies to Claude Code, Cursor, GitHub Copilot and similar assistants; the example below uses Claude Code.

## Setup

### 1. Install ghr

See the [README installation section](../README.md#installation), then verify:

```bash
which ghr && ghr --version
```

### 2. Add rules to `~/.claude/CLAUDE.md`

```
# GitHub Repository Analysis Priority

ghr is free and unlimited; prefer it over MCP tools for public repositories.

Trigger rules (user input matches → use ghr):
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

MCP fallback conditions (only use MCP when):
1. Private repository (ghr only supports public repositories)
2. Git history needed
3. User explicitly requests MCP
```

### 3. Use it

```
You: "I want to understand Next.js architecture"
Claude Code: [runs ghr analyze vercel/next.js, then answers]
```

## Working patterns

Work broad to narrow: `ghr structure` to orient, `ghr search` to locate, `ghr read` to study. For example:

```
You: "How is TypeScript organized?"
→ ghr structure microsoft/TypeScript --depth 2

You: "Where is the compiler entry point?"
→ ghr search microsoft/TypeScript "createCompiler" -e .ts

You: "Show me the core loop"
→ ghr read microsoft/TypeScript src/compiler/tsc.ts
```

Use the cache: the first command on a repository clones it, later ones read from `~/.ghr-cache`; add `--no-cache` when you need the latest commit. Use `-o file.json` when you want the assistant to reference structured output, for example `@react-analysis.json Explain the codebase architecture`.

## Troubleshooting

- **Assistant doesn't use ghr**: confirm the priority section exists in `~/.claude/CLAUDE.md` and start a new conversation so it loads.
- **`ghr` command not found**: add `~/.cargo/bin` to `PATH`, or reinstall per the README.
- **Assistant uses MCP instead**: expected for private repositories, when git history is needed, or on explicit request — ghr only supports public repositories.
