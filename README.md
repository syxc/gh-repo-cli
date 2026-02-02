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

This tool is designed to work seamlessly with AI coding assistants. **The best practice is to add simple instructions to your CLAUDE.md global configuration**, letting AI automatically detect when to use gh-repo-cli.

##### 🤖 Claude Code Integration (Best Practice)

**One-Time Setup** - Add this to your `~/.claude/CLAUDE.md`:

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

**Now just ask questions naturally**:

```
You: How does React implement hooks?

Claude Code:
$ ghr analyze facebook/react
$ ghr search facebook/react "useState" -e .js
$ ghr read facebook/react packages/react/src/ReactHooks.js

Based on my analysis, here's how React hooks are implemented...
```

**Why This Approach is Superior**:
- ✅ **Zero configuration** - No skill files to create
- ✅ **Automatic detection** - AI decides when to use gh-repo-cli
- ✅ **Natural interaction** - Ask questions, don't give commands
- ✅ **Smart fallback** - Automatically uses MCP for private repos
- ✅ **Always active** - Works for all conversations

📖 **See [AI_INTEGRATION_GUIDE.md](AI_INTEGRATION_GUIDE.md)** for detailed examples, advanced workflows, and troubleshooting.

##### 🔄 Other AI Assistants

**Cursor / Windsurf / Copilot**:
```bash
# Analyze repo in terminal
ghr analyze vuejs/core -o vue-analysis.json

# Reference output in AI chat
@vue-analysis.json Explain Vue's reactivity system
```

**ChatGPT / Claude (Web)**:
```bash
# Export repo data
ghr analyze tensorflow/tensorflow -o tf.json

# Upload JSON file and ask questions
```

##### 📊 MCP vs CLI Comparison

| Feature | MCP Servers | gh-repo-cli + AI |
|---------|-------------|-----------------|
| **Usage Limits** | ❌ Often limited (100-500/month) | ✅ Unlimited |
| **Setup** | ⚠️ Configure tokens/servers | ✅ One CLAUDE.md snippet |
| **Privacy** | ⚠️ Code goes through server | ✅ Local analysis |
| **Cost** | 💰 Paid/Quota-limited | ✅ Free |
| **Speed** | ⚠️ Network dependent | ⚡ Local cache |
| **AI Detection** | ❌ Manual invocation | ✅ Automatic |

##### 📊 MCP vs CLI Comparison

| Feature | MCP Servers | gh-repo-cli + AI |
|---------|-------------|-----------------|
| **Usage Limits** | ❌ Often limited (100-500/month) | ✅ Unlimited |
| **API Token** | ❌ Required | ✅ Not needed |
| **Privacy** | ⚠️ Code goes through server | ✅ Local analysis |
| **Cost** | 💰 Paid/Quota-limited | ✅ Free |
| **Speed** | ⚠️ Network dependent | ⚡ Local cache |
| **AI Integration** | ✅ Seamless | ✅ JSON/CLI |
| **Flexibility** | ❌ Fixed format | ✅ Custom workflows |
| **Offline** | ❌ Requires internet | ✅ Works with cached repos |

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

- 🤖 **[AI Integration Guide](AI_INTEGRATION_GUIDE.md)** - Complete guide on integrating gh-repo-cli with Claude Code, Cursor, and other AI assistants
- 🚀 **[Release Workflow Guide](RELEASE_WORKFLOW.md)** - How to configure and trigger automated releases

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
