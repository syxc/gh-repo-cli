# AI Assistant Integration

> Let the assistant decide when to use ghr, instead of invoking it by hand.

ghr clones public repositories with Git and analyzes them locally, without using GitHub API or MCP quotas. It still needs network access to clone and refresh repositories. The example below uses Claude Code; the same rules can be added to Codex or Pi instructions.

## Setup

### 1. Install ghr

See the [README installation section](../README.md#installation), then verify:

```bash
which ghr && ghr --version
```

### 2. Add the rules to the agent's instruction file

For Claude Code, edit `~/.claude/CLAUDE.md`; other agents such as Codex read their own instruction file (`AGENTS.md`). The rules to add:

```
# GitHub Repository Analysis Priority

Prefer ghr for public repository files; it uses Git instead of GitHub API or MCP quotas.

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

The first command clones the repository; later commands refresh the cached copy from the current default branch. `--no-cache` forces a new clone. Commands sharing a cache run one at a time; avoid parallel calls to the same cache. Use `-o file.json` for structured output, for example `@react-analysis.json Explain the codebase architecture`.

## Troubleshooting

- **Assistant doesn't use ghr**: confirm the priority section exists in `~/.claude/CLAUDE.md` and start a new conversation so it loads.
- **`ghr` command not found**: add `~/.cargo/bin` to `PATH`, or reinstall per the README.
- **Assistant uses MCP instead**: expected for private repositories, when git history is needed, or on explicit request — ghr only supports public repositories.
