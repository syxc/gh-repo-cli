# AI Integration Guide for gh-repo-cli

> Complete guide on how to integrate gh-repo-cli with AI coding assistants for optimal workflow

## 📚 Table of Contents

- [Why Integrate gh-repo-cli with AI?](#why-integrate-gh-repo-cli-with-ai)
- [Claude Code Integration](#claude-code-integration)
- [Cursor Integration](#cursor-integration)
- [Other AI Assistants](#other-ai-assistants)
- [Advanced Workflows](#advanced-workflows)
- [Best Practices](#best-practices)

---

## Why Integrate gh-repo-cli with AI?

### The Problem with MCP Servers

Many AI coding assistants provide GitHub repository analysis through MCP (Model Context Protocol) servers. However, these have significant limitations:

| Issue | Impact | Solution with gh-repo-cli |
|-------|--------|---------------------------|
| **Usage Quotas** | 100-500 requests/month | ✅ Unlimited usage |
| **Rate Limits** | 60 requests/hour (GitHub API) | ✅ No API calls |
| **Cost** | $10-50/month for premium | ✅ Completely free |
| **Privacy** | Code sent to third-party servers | ✅ Local analysis |
| **Reliability** | Server downtime affects work | ✅ Works offline (cached repos) |

### The Best of Both Worlds

**gh-repo-cli + AI Assistant** gives you:

1. **Unlimited Analysis**: Analyze as many repos as you want
2. **Local Privacy**: Code never leaves your machine
3. **Fast Performance**: Local cache = instant re-analysis
4. **AI Intelligence**: Get deep insights from LLMs
5. **Cost Effective**: $0 vs $50/month

---

## Claude Code Integration

### Method 1: Manual Workflow (Quick Start)

**Scenario**: Understand how a specific feature works in a large codebase.

#### Step 1: Analyze Repository Structure

```bash
ghr analyze facebook/react -o react-analysis.json
```

This creates a comprehensive JSON file with:
- Repository metadata
- Language breakdown
- Directory structure
- File statistics
- Dependencies

#### Step 2: Search for Relevant Code

```bash
ghr search facebook/react useState -e .js -o usestate-results.json
```

This finds all files containing `useState` in JavaScript files.

#### Step 3: Read Specific Files

```bash
ghr read facebook/react packages/react/src/ReactHooks.js
```

Get the full content of specific files.

#### Step 4: Provide Context to Claude Code

```
I'm analyzing React's useState implementation. Here's what I found:

Repository Analysis:
[paste react-analysis.json]

Search Results for useState:
[paste usestate-results.json]

Key File (ReactHooks.js):
[paste output from ghr read]

Can you explain:
1. How useState initializes state
2. The hook dispatcher mechanism
3. How it tracks component state across re-renders
```

**Why This Works**:
- Claude gets structured, comprehensive context
- You control exactly what information to share
- No quota limits - analyze as many files as needed

---

### Method 2: Automated Workflow (Recommended)

Create a **custom skill** for Claude Code to automate repository analysis.

#### Step 1: Create Skill Configuration

Create file: `~/.claude/skills/github-repo-analyzer.json`

```json
{
  "name": "github-repo-analyzer",
  "displayName": "GitHub Repository Analyzer",
  "description": "Analyze GitHub repositories using gh-repo-cli (ghr) without API limits",
  "version": "1.0.0",
  "author": "your-name",
  "command": "ghr",
  "arguments": {
    "analyze": {
      "template": ["analyze", "{{repo}}", "-o", "{{output}}"],
      "description": "Analyze a GitHub repository",
      "parameters": {
        "repo": {
          "type": "string",
          "description": "Repository in owner/repo format",
          "required": true,
          "examples": ["facebook/react", "vuejs/core", "tensorflow/tensorflow"]
        },
        "output": {
          "type": "string",
          "description": "Output JSON file path",
          "default": "analysis.json"
        }
      }
    },
    "search": {
      "template": ["search", "{{repo}}", "{{query}}", "-o", "{{output}}"],
      "description": "Search for code patterns in a repository",
      "parameters": {
        "repo": {
          "type": "string",
          "description": "Repository in owner/repo format",
          "required": true
        },
        "query": {
          "type": "string",
          "description": "Search query (supports regex)",
          "required": true
        },
        "output": {
          "type": "string",
          "description": "Output JSON file path",
          "default": "search-results.json"
        }
      }
    },
    "read": {
      "template": ["read", "{{repo}}", "{{file}}"],
      "description": "Read a specific file from a repository",
      "parameters": {
        "repo": {
          "type": "string",
          "description": "Repository in owner/repo format",
          "required": true
        },
        "file": {
          "type": "string",
          "description": "File path within the repository",
          "required": true
        }
      }
    },
    "structure": {
      "template": ["structure", "{{repo}}", "--depth", "{{depth}}", "-o", "{{output}}"],
      "description": "Get repository directory structure",
      "parameters": {
        "repo": {
          "type": "string",
          "description": "Repository in owner/repo format",
          "required": true
        },
        "depth": {
          "type": "number",
          "description": "Directory tree depth",
          "default": 3
        },
        "output": {
          "type": "string",
          "description": "Output JSON file path",
          "default": "structure.json"
        }
      }
    }
  },
  "examples": [
    {
      "description": "Analyze React repository",
      "command": "analyze facebook/react -o react-analysis.json"
    },
    {
      "description": "Search for useState in React",
      "command": "search facebook/react useState -o usestate.json"
    },
    {
      "description": "Read ReactHooks.js file",
      "command": "read facebook/react packages/react/src/ReactHooks.js"
    }
  ]
}
```

#### Step 2: Use the Skill in Claude Code

Now you can simply ask Claude Code:

```
Please analyze the Vue.js core repository using the github-repo-analyzer skill.
I want to understand:
1. The overall project structure
2. How reactivity is implemented
3. The compiler architecture
```

**Claude Code will automatically**:
1. Run `ghr analyze vuejs/core`
2. Parse the JSON output
3. Run additional searches as needed
4. Read relevant files
5. Synthesize a comprehensive analysis

#### Step 3: Advanced Prompt Template

For even better results, use this prompt template:

```
Analyze {{owner}}/{{repo}} using the github-repo-analyzer skill.

Focus on:
1. {{feature_1}}
2. {{feature_2}}
3. {{feature_3}}

Workflow:
1. Get repository structure (depth: 3)
2. Search for relevant code patterns
3. Read key implementation files
4. Provide architectural overview

Output format:
- High-level architecture
- Key files and their roles
- Code flow explanation
- Notable patterns or best practices
```

**Example usage**:
```
Analyze facebook/react using the github-repo-analyzer skill.

Focus on:
1. Hooks implementation
2. Reconciliation algorithm
3. Fiber architecture

[rest of template...]
```

---

### Method 3: Shell Integration (Power Users)

Add helper functions to your shell for seamless Claude Code integration.

#### Bash/Zsh Functions

Add to `~/.bashrc` or `~/.zshrc`:

```bash
# gh-repo-cli helper functions for Claude Code

# Analyze repo and copy to clipboard
ghr-analyze() {
    local repo=$1
    local output=${2:-"/tmp/ghr-analysis.json"}

    ghr analyze "$repo" -o "$output"
    echo "Analysis saved to: $output"

    # Copy to clipboard (macOS)
    if command -v pbcopy &> /dev/null; then
        cat "$output" | pbcopy
        echo "✅ JSON copied to clipboard (paste into Claude Code)"
    fi
}

# Search and copy results
ghr-search() {
    local repo=$1
    local query=$2
    local output=${3:-"/tmp/ghr-search.json"}

    ghr search "$repo" "$query" -o "$output"
    echo "Search results saved to: $output"

    if command -v pbcopy &> /dev/null; then
        cat "$output" | pbcopy
        echo "✅ Results copied to clipboard"
    fi
}

# Read file and copy to clipboard
ghr-read() {
    local repo=$1
    local file=$2

    local content=$(ghr read "$repo" "$file")

    if command -v pbcopy &> /dev/null; then
        echo "$content" | pbcopy
        echo "✅ File content copied to clipboard"
    fi

    echo "$content"
}

# Full analysis workflow
ghr-workflow() {
    local repo=$1
    local query=$2

    echo "🔍 Analyzing $repo..."

    # Step 1: Structure
    ghr structure "$repo" --depth 3 -o /tmp/ghr-structure.json

    # Step 2: Analyze
    ghr analyze "$repo" -o /tmp/ghr-analysis.json

    # Step 3: Search (if query provided)
    if [[ -n "$query" ]]; then
        ghr search "$repo" "$query" -o /tmp/ghr-search.json
    fi

    echo "✅ Analysis complete!"
    echo "📁 Structure: /tmp/ghr-structure.json"
    echo "📊 Analysis: /tmp/ghr-analysis.json"

    if [[ -n "$query" ]]; then
        echo "🔎 Search: /tmp/ghr-search.json"
    fi

    # Combine and copy to clipboard
    echo "{" > /tmp/ghr-combined.json
    echo "  \"structure\": $(cat /tmp/ghr-structure.json)," >> /tmp/ghr-combined.json
    echo "  \"analysis\": $(cat /tmp/ghr-analysis.json)" >> /tmp/ghr-combined.json

    if [[ -n "$query" ]]; then
        echo "  ,\"search\": $(cat /tmp/ghr-search.json)" >> /tmp/ghr-combined.json
    fi

    echo "}" >> /tmp/ghr-combined.json

    cat /tmp/ghr-combined.json | pbcopy
    echo "📋 Combined JSON copied to clipboard - paste into Claude Code!"
}
```

**Usage**:

```bash
# Quick analysis
ghr-analyze facebook/react

# Search for specific code
ghr-search facebook/react useState

# Full workflow (combines structure + analysis + search)
ghr-workflow vuejs/core "reactive"

# Now paste into Claude Code and ask questions!
```

---

## Cursor Integration

Cursor has built-in terminal integration, making it perfect for gh-repo-cli.

### Workflow 1: Inline Terminal

1. **Open Cursor's terminal** (⌘+` on macOS)
2. **Run gh-repo-cli commands**:

```bash
ghr analyze tensorflow/tensorflow -o tf.json
```

3. **Reference the file in Cursor**:

```
@tf.json I'm analyzing TensorFlow. Can you explain the core architecture
based on this repository analysis?
```

### Workflow 2: Chat Mode

1. **Run analysis in terminal**:

```bash
ghr search vuejs/core "scheduler" -e .ts -o scheduler.json
```

2. **Ask Cursor with context**:

```
Analyze the Vue.js scheduler implementation:

@scheduler.json

Key questions:
- How does the scheduler prioritize jobs?
- What's the time-slicing mechanism?
- How does it handle async tasks?
```

---

## Other AI Assistants

### ChatGPT / Claude (Web)

```bash
# 1. Export repository data
ghr analyze facebook/react -o react.json
ghr search facebook/react "fiber" -e .js -o fiber.json

# 2. Upload files to ChatGPT/Claude
# 3. Ask questions with context

"I'm studying React's Fiber architecture. Here's the repository analysis
and search results for 'fiber'. Can you explain..."
```

### GitHub Copilot

```bash
# Use gh-repo-cli in VS Code terminal
ghr analyze nextjs/next.js -o nextjs.json

# Copilot Chat will see the terminal output
# Ask: "Explain Next.js routing based on the analysis above"
```

---

## Advanced Workflows

### Workflow 1: Comparative Analysis

Compare two repositories to understand different approaches:

```bash
# Analyze both repos
ghr analyze facebook/react -o react.json
ghr analyze vuejs/core -o vue.json

# Ask Claude:
"I want to compare React and Vue's reactivity systems.

React Analysis:
[paste react.json]

Vue Analysis:
[paste vue.json]

Can you compare:
1. State management approaches
2. Component lifecycle handling
3. Performance optimization strategies
"
```

### Workflow 2: Migration Planning

Plan a migration from one library to another:

```bash
# Analyze current usage
ghr search your-org/your-repo "moment.js" -o moment-usage.json

# Analyze target library
ghr analyze moment/moment -o moment-lib.json

# Ask Claude for migration plan:
"We're migrating from moment.js to date-fns.

Current moment.js usage:
[paste moment-usage.json]

moment library structure:
[paste moment-lib.json]

Create a migration plan with:
1. API mapping (moment → date-fns)
2. Breaking changes to handle
3. Step-by-step migration checklist
"
```

### Workflow 3: Bug Investigation

Investigate a bug by analyzing the repository:

```bash
# Reproduce the issue locally
ghr analyze facebook/react -o react.json
ghr search facebook/react "useEffect cleanup" -e .js -o cleanup.json

# Get specific file
ghr read facebook/react packages/react/src/ReactHooks.js

# Ask Claude:
"I'm investigating a useEffect cleanup bug.

Repository info:
[paste react.json]

Search results:
[paste cleanup.json]

ReactHooks.js:
[paste file content]

Symptom: Cleanup function not running on unmount
What could cause this and how to fix?
"
```

---

## Best Practices

### 1. Start with Structure

Always get the repository structure first:

```bash
ghr structure owner/repo --depth 3 -o structure.json
```

This helps you understand the codebase layout before diving deeper.

### 2. Use Specific Queries

Narrow down searches with specific terms:

```bash
# Good
ghr search facebook/react "useState.*dispatch" -o results.json

# Too broad
ghr search facebook/react "useState" -o results.json
```

### 3. Combine Multiple Commands

Don't rely on a single command - combine them:

```bash
# 1. High-level view
ghr analyze owner/repo -o analysis.json

# 2. Targeted search
ghr search owner/repo "specific-term" -o search.json

# 3. Deep dive into files
ghr read owner/repo path/to/file.js
```

### 4. Cache is Your Friend

Remember that repositories are cached in `~/.ghr-cache/`:

```bash
# First run: clones the repo (slower)
ghr analyze facebook/react

# Subsequent runs: uses cache (instant)
ghr analyze facebook/react --no-cache  # force refresh if needed
```

### 5. JSON Output for AI

Always use `-o` flag when working with AI assistants:

```bash
# Good: AI can parse JSON
ghr analyze owner/repo -o analysis.json

# Less ideal: AI has to parse text
ghr analyze owner/repo
```

### 6. Respect Repository Size

For very large repositories (monorepos, massive projects):

```bash
# Limit depth
ghr structure microsoft/vscode --depth 2

# Search specific directories
ghr search facebook/react "useState" -d packages/react

# Read specific files only
ghr read facebook/react README.md
```

---

## Troubleshooting

### Issue: Claude Code can't find `ghr` command

**Solution**: Make sure `ghr` is in your PATH:

```bash
# Check if ghr is installed
which ghr

# If not found, reinstall
npm link
```

### Issue: JSON output is too large for Claude

**Solution**: Split the analysis:

```bash
# Get just the structure
ghr structure owner/repo -o structure.json

# Search for specific topics
ghr search owner/repo "topic" -o search.json

# Share relevant parts with Claude
```

### Issue: Repository is too large to clone

**Solution**: Use partial clone or specific branch:

```bash
# This is a limitation - gh-repo-cli clones full repos
# For large repos, consider:
# 1. Using --depth 1 in git manually
# 2. Focusing on specific subdirectories
# 3. Using GitHub web interface + gh-repo-cli for files
```

---

## Conclusion

Integrating **gh-repo-cli** with AI assistants gives you:

- ✅ **Unlimited** repository analysis
- ✅ **Private** local processing
- ✅ **Fast** cached performance
- ✅ **Flexible** custom workflows
- ✅ **Free** alternative to paid MCP servers

**Next Steps**:

1. Install gh-repo-cli: `npm install -g gh-repo-cli`
2. Set up the Claude Code skill (see Method 2 above)
3. Try the examples with your favorite repositories
4. Customize workflows for your specific needs

**Questions?** Open an issue on GitHub: https://github.com/syxc/gh-repo-cli/issues

---

<div align="center">

**Happy coding! 🚀**

**Unlimited repo analysis + AI intelligence = Supercharged development**

</div>
