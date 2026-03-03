# gh-repo-cli

> A lightweight CLI tool for analyzing GitHub repositories without API tokens

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Node.js Version](https://img.shields.io/node/v/gh-repo-cli)](https://github.com/syxc/gh-repo-cli)
[![CI](https://github.com/syxc/gh-repo-cli/workflows/CI/badge.svg)](https://github.com/syxc/gh-repo-cli/actions)
[![Code Quality](https://github.com/syxc/gh-repo-cli/workflows/Code%20Quality/badge.svg)](https://github.com/syxc/gh-repo-cli/actions)

Analyze, search, and explore GitHub repositories from your terminal - no API token required.

## 🎯 Why This Tool?

### The Problem

Many AI coding assistants (like GLM Coding Plan, Claude Code, Cursor, etc.) provide GitHub repository analysis features through **MCP (Model Context Protocol)** servers or built-in tools. However, these services often have **usage quotas** that limit your productivity:

- **GLM Coding Lite**: Limited API calls per month
- **GitHub API**: Rate limits (60 requests/hour for unauthenticated)
- **MCP Servers**: Often have daily/monthly quotas
- **Paid Tools**: Expensive subscriptions for heavy usage

### The Solution

**gh-repo-cli** is a **free, unlimited alternative** that:

- ✅ Uses `git clone` instead of GitHub API - **no rate limits**
- ✅ Works **standalone** or **with any AI assistant**
- ✅ Caches repositories locally for **fast re-analysis**
- ✅ Supports **proxies** for accessing GitHub from anywhere
- ✅ Provides **structured JSON output** for easy AI parsing

### Use Cases

#### 1. Standalone CLI Tool

Use it directly in your terminal for quick repository analysis:

```bash
ghr analyze facebook/react
ghr search vuejs/core ref
ghr read facebook/react README.md
```

#### 2. With AI Coding Assistants (Recommended!)

**Best Practice**: Add gh-repo-cli usage rules to `~/.claude/CLAUDE.md` global configuration, let AI automatically detect when repository analysis is needed.

**One-Step Setup** - Add to `~/.claude/CLAUDE.md`:

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

**Start natural conversation after configuration**:

```
You: "How does React implement hooks?"

Claude Code:
  $ ghr analyze facebook/react
  $ ghr search facebook/react "useState" -e .js
  $ ghr read facebook/react packages/react/src/ReactHooks.js

  Based on repository analysis, React hooks implementation is...
```

**Advantages**:
- ✅ **Zero config** - No need to create skill files
- ✅ **Auto detection** - AI decides when to use gh-repo-cli
- ✅ **Natural interaction** - Ask in natural language, no manual command invocation
- ✅ **Smart fallback** - Automatically use MCP for private repositories
- ✅ **Always active** - Works in all conversations

📖 **Complete Guide**: [docs/AI_INTEGRATION.md](docs/AI_INTEGRATION.md) - Detailed examples, advanced workflows, and troubleshooting

#### 3. Other AI Assistants

**Cursor / Windsurf / Copilot**:
```bash
# Analyze repository in terminal
ghr analyze vuejs/core -o vue-analysis.json

# Reference output in AI chat
@vue-analysis.json Explain Vue's reactivity system
```

**ChatGPT / Claude (Web)**:
```bash
# Export repository data
ghr analyze tensorflow/tensorflow -o tf.json

# Upload JSON file and ask questions
```

#### 4. MCP vs CLI Comparison

| Feature | MCP Servers | gh-repo-cli |
|---------|-------------|-------------|
| **Usage Limits** | ❌ Often limited | ✅ Unlimited |
| **Setup** | ⚠️ Configure tokens/servers | ✅ One CLAUDE.md snippet |
| **Privacy** | ⚠️ Code goes through server | ✅ Local analysis |
| **Cost** | 💰 Paid/Quota-limited | ✅ Free |
| **Speed** | ⚠️ Network dependent | ⚡ Local cache |
| **AI Detection** | ❌ Manual invocation | ✅ Automatic |

## ✨ Features

- 🔍 **No API Token Required** - Uses git clone instead of GitHub API
- 📊 **Comprehensive Analysis** - Language detection, file statistics, directory structure
- 🔎 **Code Search** - Search for patterns across the entire codebase
- 📁 **File Operations** - Read files, list directories
- 🌐 **Proxy Support** - Works with HTTP/HTTPS/SOCKS5 proxies
- ⚡ **Local Cache** - Repositories are cached for faster subsequent access
- 🔒 **Secure** - No data leaves your machine except git clone operations
- 🤖 **AI-Friendly** - JSON output format for easy integration with AI assistants

## 📦 Installation

```bash
# Install via npm (recommended)
npm install -g @syxc/gh-repo-cli

# Or clone and link manually
git clone https://github.com/syxc/gh-repo-cli.git
cd gh-repo-cli
npm install
npm link
```

## 🚀 Usage

### Basic Commands

```bash
# Analyze a repository
ghr analyze facebook/react

# Get directory structure
ghr structure vuejs/core

# Search for code patterns
ghr search facebook/react useState

# Read a specific file
ghr read facebook/react README.md

# List files in a directory
ghr ls facebook/react/src

# Clean cached repositories
ghr clean --all              # Clean all cached repos
ghr clean facebook/react     # Clean specific repo
```

### AI-Assisted Workflow (Recommended)

```bash
# Step 1: Explore the repository
ghr structure facebook/react --depth 2

# Step 2: Search for specific patterns
ghr search facebook/react useEffect -e .js -o search_results.json

# Step 3: Read relevant files
ghr read facebook/react packages/react/src/ReactHooks.js

# Step 4: Share findings with your AI assistant
# (Claude Code, Cursor, Copilot, etc.)
```

## 🌐 Proxy Support

If you're behind a firewall or need to access GitHub through a proxy:

```bash
# Set proxy environment variable
export GH_PROXY="http://127.0.0.1:7890"

# Or use per-command
GH_PROXY="http://127.0.0.1:7890" ghr analyze facebook/react
```

Supported proxy types:
- HTTP/HTTPS proxy: `http://127.0.0.1:7890`
- SOCKS5 proxy: `socks5://127.0.0.1:1080`
- With authentication: `http://username:password@proxy.example.com:8080`

## 📚 Advanced Usage

```bash
# Save output to file (great for AI analysis!)
ghr analyze facebook/react -o output.json

# Search with file extension filter
ghr search facebook/react useEffect -e .js

# Case-insensitive search
ghr search facebook/react types --ignore-case

# Bypass cache and re-clone
ghr analyze facebook/react --no-cache

# Get deeper directory structure
ghr structure facebook/react --depth 4
```

## 🔧 Configuration

### Cache Location

Repositories are cached in `~/.ghr-cache/`:

```bash
# Clear cache for a specific repo (using CLI)
ghr clean facebook/react

# Clear all cache (using CLI)
ghr clean --all

# Or manually
rm -rf ~/.ghr-cache/facebook/react
rm -rf ~/.ghr-cache/
```

### Output Location

Analysis results are saved in `~/.ghr-output/` when using the `-o` option.

## 📖 Additional Documentation

- 🤖 **[AI Integration Guide](docs/AI_INTEGRATION.md)** - Best practices for Claude Code integration
- 🚀 **[Release Workflow Guide](docs/RELEASE_WORKFLOW.md)** - Automated release configuration guide
- 🧪 **[Testing Guide](docs/TESTING.md)** - Testing guide

## 🤝 Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details.

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [commander](https://github.com/tj/commander.js)
- Created to save API quota usage and provide unlimited repository analysis
- Inspired by the need for free, private, and unlimited GitHub repository exploration

---

<div align="center">

**Made with ❤️ by the open-source community**

**Tired of API quotas?** ⚡ Use gh-repo-cli + your favorite AI assistant!

</div>
