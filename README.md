# gh-repo-cli

> A lightweight CLI tool for analyzing GitHub repositories without API tokens

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

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
gh analyze facebook/react
gh search vuejs/core ref
gh read facebook/react README.md
```

#### 2. With AI Coding Assistants (Recommended!)

This tool shines when combined with AI assistants. Here's how to use it with popular tools:

##### 🤖 Claude Code Integration (Example)

**Scenario**: You want to understand how React's `useState` hook works.

**Step 1**: Get repository structure
```bash
gh structure facebook/react --depth 3
```

**Step 2**: Search for the implementation
```bash
gh search facebook/react useState -e .js -o results.json
```

**Step 3**: Read the relevant file
```bash
gh read facebook/react packages/react/src/ReactHooks.js
```

**Step 4**: Ask Claude Code
```
I've analyzed the React repository structure and found the useState implementation
in ReactHooks.js. Can you explain how it works internally?

Here's the file content:
[paste the output from gh read command]
```

**Why This Works Better**:
- ⚡ **No quota limits** - analyze as many repos as you want
- 🔒 **Privacy** - code stays on your machine until you share it
- 💰 **Cost-effective** - free vs paid MCP servers
- 🎯 **Focused** - get exactly what you need, then ask AI specific questions

##### 🔄 Comparison: MCP Servers vs gh-repo-cli

| Feature | MCP Servers | gh-repo-cli |
|---------|-------------|-------------|
| **Usage Limits** | ❌ Often limited | ✅ Unlimited |
| **API Token** | ❌ Required | ✅ Not needed |
| **Privacy** | ⚠️ Code goes through server | ✅ Local analysis |
| **Cost** | 💰 Paid/Quota-limited | ✅ Free |
| **Speed** | ⚠️ Network dependent | ⚡ Local cache |
| **AI Integration** | ✅ Seamless | ✅ Copy-paste/CLI |

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
# Clone the repository
git clone https://github.com/syxc/gh-repo-cli.git
cd gh-repo-cli

# Install dependencies
npm install

# Link globally
npm link
```

## 🚀 Usage

### Basic Commands

```bash
# Analyze a repository
gh analyze facebook/react

# Get directory structure
gh structure vuejs/core

# Search for code patterns
gh search facebook/react useState

# Read a specific file
gh read facebook/react README.md

# List files in a directory
gh ls facebook/react/src
```

### AI-Assisted Workflow (Recommended)

```bash
# Step 1: Explore the repository
gh structure facebook/react --depth 2

# Step 2: Search for specific patterns
gh search facebook/react useEffect -e .js -o search_results.json

# Step 3: Read relevant files
gh read facebook/react packages/react/src/ReactHooks.js

# Step 4: Share findings with your AI assistant
# (Claude Code, Cursor, Copilot, etc.)
```

## 🌐 Proxy Support

If you're behind a firewall or need to access GitHub through a proxy:

```bash
# Set proxy environment variable
export GH_PROXY="http://127.0.0.1:7890"

# Or use per-command
GH_PROXY="http://127.0.0.1:7890" gh analyze facebook/react
```

Supported proxy types:
- HTTP/HTTPS proxy: `http://127.0.0.1:7890`
- SOCKS5 proxy: `socks5://127.0.0.1:1080`
- With authentication: `http://username:password@proxy.example.com:8080`

## 📚 Advanced Usage

```bash
# Save output to file (great for AI analysis!)
gh analyze facebook/react -o output.json

# Search with file extension filter
gh search facebook/react useEffect -e .js

# Case-insensitive search
gh search facebook/react types --ignore-case

# Bypass cache and re-clone
gh analyze facebook/react --no-cache

# Get deeper directory structure
gh structure facebook/react --depth 4
```

## 🔧 Configuration

### Cache Location

Repositories are cached in `~/.gh-cli-cache/`:

```bash
# Clear cache for a specific repo
rm -rf ~/.gh-cli-cache/facebook/react

# Clear all cache
rm -rf ~/.gh-cli-cache/
```

### Output Location

Analysis results are saved in `~/.gh-cli-output/` when using the `-o` option.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

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
