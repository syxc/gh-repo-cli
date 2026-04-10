# ghr (gh-repo-cli)

> A lightweight CLI tool for analyzing GitHub repositories without API tokens

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Go Version](https://img.shields.io/badge/go-%3E%3D1.23-blue)](https://github.com/syxc/gh-repo-cli)
[![CI](https://github.com/syxc/gh-repo-cli/workflows/CI/badge.svg)](https://github.com/syxc/gh-repo-cli/actions)
[![Code Quality](https://github.com/syxc/gh-repo-cli/workflows/Code%20Quality/badge.svg)](https://github.com/syxc/gh-repo-cli/actions)

Analyze, search, and explore GitHub repositories from your terminal - no API token required.

## 🎯 Why This Tool?

### The Problem

Many AI coding assistants (like Claude Code, Cursor, Copilot, etc.) provide GitHub repository analysis features through **MCP (Model Context Protocol)** servers or built-in tools. However, these services often have **usage quotas** that limit your productivity:

- **GitHub API**: Rate limits (60 requests/hour for unauthenticated)
- **MCP Servers**: Often have daily/monthly quotas
- **Paid Tools**: Expensive subscriptions for heavy usage

### The Solution

**ghr** is a **free, unlimited alternative** that:

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

**Best Practice**: Add ghr usage rules to your AI assistant's configuration, let AI automatically detect when repository analysis is needed.

**One-Step Setup** - Add to `~/.claude/CLAUDE.md`:

```
# GitHub Repository Analysis Priority

MCP (zread): Limited quota ❌
ghr: Completely free ✅

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

**Start natural conversation after configuration**:

```
You: "How does React implement hooks?"

AI Assistant:
  $ ghr analyze facebook/react
  $ ghr search facebook/react "useState" -e .js
  $ ghr read facebook/react packages/react/src/ReactHooks.js

  Based on repository analysis, React hooks implementation is...
```

**Advantages**:
- ✅ **Zero config** - No need to create skill files
- ✅ **Auto detection** - AI decides when to use ghr
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

| Feature | MCP Servers | ghr |
|---------|-------------|-----|
| **Usage Limits** | ❌ Often limited | ✅ Unlimited |
| **Setup** | ⚠️ Configure tokens/servers | ✅ Install and go |
| **Privacy** | ⚠️ Code goes through server | ✅ Local analysis |
| **Cost** | 💰 Paid/Quota-limited | ✅ Free |
| **Speed** | ⚠️ Network dependent | ⚡ Local cache |
| **AI Detection** | ❌ Manual invocation | ✅ Automatic |

## ✨ Features

- 🔍 **No API Token Required** - Uses git clone instead of GitHub API
- 📊 **Comprehensive Analysis** - Language detection, file statistics, directory structure
- 🔎 **Code Search** - Search for patterns across the entire codebase
- 📁 **File Operations** - Read files, list directories
- 🌐 **Proxy Support** - Works with HTTP/HTTPS proxies
- ⚡ **Local Cache** - Repositories are cached for faster subsequent access
- 🔒 **Secure** - No data leaves your machine except git clone operations
- 🤖 **AI-Friendly** - JSON output format for easy integration with AI assistants
- 🚀 **Fast** - Written in Go for optimal performance

## 📦 Installation

### Option 1: Using go install (Recommended)

**Install:**
```bash
go install github.com/syxc/gh-repo-cli/cmd/ghr@latest
```

**Add to PATH (first time only):**

macOS (zsh):
```bash
# Add to ~/.zshenv (create if not exists)
echo 'export PATH=$PATH:$(go env GOPATH)/bin' >> ~/.zshenv
source ~/.zshenv
```

Linux (bash):
```bash
# Add to ~/.bashrc
echo 'export PATH=$PATH:$(go env GOPATH)/bin' >> ~/.bashrc
source ~/.bashrc
```

Fish:
```bash
set -Ux PATH $PATH (go env GOPATH)/bin
```

### Option 2: Download Pre-built Binary

Download the latest release for your platform from the [Releases](https://github.com/syxc/gh-repo-cli/releases) page.

```bash
# Example: macOS ARM64
curl -L -o ghr "https://github.com/syxc/gh-repo-cli/releases/latest/download/ghr-$(uname -s)-$(uname -m)"
chmod +x ghr
sudo mv ghr /usr/local/bin/
```

### Option 3: Build from Source

```bash
git clone https://github.com/syxc/gh-repo-cli.git
cd ghr
go build -o ghr .
sudo mv ghr /usr/local/bin/
```

### Option 4: Download Pre-built Binary

Download the latest release for your platform:

```bash
# macOS ARM64 (Apple Silicon)
curl -L -o ghr.tar.gz "https://github.com/syxc/gh-repo-cli/releases/latest/download/ghr_$(curl -s https://api.github.com/repos/syxc/gh-repo-cli/releases/latest | grep tag_name | cut -d'"' -f4)_darwin_arm64.tar.gz"
tar -xzf ghr.tar.gz
sudo mv ghr /usr/local/bin/

# Linux AMD64
curl -L -o ghr.tar.gz "https://github.com/syxc/gh-repo-cli/releases/latest/download/ghr_$(curl -s https://api.github.com/repos/syxc/gh-repo-cli/releases/latest | grep tag_name | cut -d'"' -f4)_linux_amd64.tar.gz"
tar -xzf ghr.tar.gz
sudo mv ghr /usr/local/bin/
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
ghr ls facebook/react src

# Read repository README
ghr readme facebook/react

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

Supported proxy environment variables (checked in order):
- `GH_PROXY` - Tool-specific proxy
- `HTTPS_PROXY` / `https_proxy` - Standard HTTPS proxy
- `HTTP_PROXY` / `http_proxy` - Standard HTTP proxy

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

# List with custom depth
ghr ls facebook/react src --depth 2
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

- 🤖 **[AI Integration Guide](docs/AI_INTEGRATION.md)** - Best practices for AI assistant integration
- 🚀 **[Release Workflow Guide](docs/RELEASE_WORKFLOW.md)** - Automated release configuration guide
- 🧪 **[Testing Guide](docs/TESTING.md)** - Testing and development guide

## 🤝 Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details.

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [Cobra](https://github.com/spf13/cobra) - The powerful CLI framework for Go
- Written in [Go](https://go.dev/) for performance and reliability
- Created to save API quota usage and provide unlimited repository analysis
- Inspired by the need for free, private, and unlimited GitHub repository exploration

---

<div align="center">

**Made with ❤️ by the open-source community**

**Tired of API quotas?** ⚡ Use ghr + your favorite AI assistant!

</div>
