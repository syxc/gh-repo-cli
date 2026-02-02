# gh-repo-cli

> 轻量级 GitHub 仓库分析命令行工具，无需 API Token

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

从终端分析、搜索和探索 GitHub 仓库 - 无需 API Token。

## 🎯 为什么需要这个工具？

### 问题所在

许多 AI 编程助手（如 GLM Coding Plan、Claude Code、Cursor 等）通过 **MCP（模型上下文协议）** 服务器或内置工具提供 GitHub 仓库分析功能。但是，这些服务通常有**使用配额限制**，影响你的工作效率：

- **GLM Coding Lite**: 每月有限的 API 调用次数
- **GitHub API**: 速率限制（未认证 60 次/小时）
- **MCP 服务器**: 通常有每日/每月配额
- **付费工具**: 高额订阅费用

### 解决方案

**gh-repo-cli** 是一个**免费、无限制的替代方案**：

- ✅ 使用 `git clone` 替代 GitHub API - **无速率限制**
- ✅ 可**独立使用**或**与任何 AI 助手配合使用**
- ✅ 本地缓存仓库，**快速重新分析**
- ✅ 支持**代理**，随时随地访问 GitHub
- ✅ 提供**结构化 JSON 输出**，便于 AI 解析

### 使用场景

#### 1. 独立命令行工具

直接在终端使用，快速分析仓库：

```bash
gh analyze facebook/react
gh search vuejs/core ref
gh read facebook/react README.md
```

#### 2. 与 AI 编程助手配合使用（推荐！）

这个工具与 AI 助手结合使用时效果最佳。以下是如何在流行的工具中使用它：

##### 🤖 Claude Code 集成示例

**场景**: 你想了解 React 的 `useState` hook 是如何工作的。

**步骤 1**: 获取仓库结构
```bash
gh structure facebook/react --depth 3
```

**步骤 2**: 搜索实现代码
```bash
gh search facebook/react useState -e .js -o results.json
```

**步骤 3**: 读取相关文件
```bash
gh read facebook/react packages/react/src/ReactHooks.js
```

**步骤 4**: 向 Claude Code 提问
```
我已经分析了 React 仓库结构，找到了 useState 的实现在 ReactHooks.js 文件中。
你能解释一下它的内部工作原理吗？

以下是文件内容：
[粘贴 gh read 命令的输出]
```

**为什么这样更好**:
- ⚡ **无配额限制** - 想分析多少仓库就分析多少
- 🔒 **隐私保护** - 代码保留在你的机器上，直到你选择分享
- 💰 **性价比高** - 免费使用 vs 付费 MCP 服务器
- 🎯 **更专注** - 精准获取所需信息，然后向 AI 提出具体问题

##### 🔄 对比：MCP 服务器 vs gh-repo-cli

| 特性 | MCP 服务器 | gh-repo-cli |
|------|-----------|-------------|
| **使用限制** | ❌ 通常受限 | ✅ 无限制 |
| **API Token** | ❌ 必需 | ✅ 不需要 |
| **隐私** | ⚠️ 代码经过服务器 | ✅ 本地分析 |
| **成本** | 💰 付费/配额限制 | ✅ 免费 |
| **速度** | ⚠️ 依赖网络 | ⚡ 本地缓存 |
| **AI 集成** | ✅ 无缝集成 | ✅ 复制粘贴/CLI |

## ✨ 特性

- 🔍 **无需 API Token** - 使用 git clone 替代 GitHub API
- 📊 **全面分析** - 语言检测、文件统计、目录结构
- 🔎 **代码搜索** - 在整个代码库中搜索模式
- 📁 **文件操作** - 读取文件、列出目录
- 🌐 **代理支持** - 支持 HTTP/HTTPS/SOCKS5 代理
- ⚡ **本地缓存** - 仓库缓存，访问更快
- 🔒 **安全** - 除 git clone 操作外，无数据离开您的机器
- 🤖 **AI 友好** - JSON 输出格式，便于与 AI 助手集成

## 📦 安装

```bash
# 克隆仓库
git clone https://github.com/syxc/gh-repo-cli.git
cd gh-repo-cli

# 安装依赖
npm install

# 全局链接
npm link
```

## 🚀 使用方法

### 基本命令

```bash
# 分析仓库
gh analyze facebook/react

# 获取目录结构
gh structure vuejs/core

# 搜索代码模式
gh search facebook/react useState

# 读取特定文件
gh read facebook/react README.md

# 列出目录文件
gh ls facebook/react/src
```

### AI 辅助工作流（推荐）

```bash
# 步骤 1: 探索仓库
gh structure facebook/react --depth 2

# 步骤 2: 搜索特定模式
gh search facebook/react useEffect -e .js -o search_results.json

# 步骤 3: 读取相关文件
gh read facebook/react packages/react/src/ReactHooks.js

# 步骤 4: 与 AI 助手分享发现
# (Claude Code、Cursor、Copilot 等)
```

## 🌐 代理支持

如果需要通过代理访问 GitHub：

```bash
# 设置代理环境变量
export GH_PROXY="http://127.0.0.1:7890"

# 或单次使用
GH_PROXY="http://127.0.0.1:7890" gh analyze facebook/react
```

支持的代理类型：
- HTTP/HTTPS 代理：`http://127.0.0.1:7890`
- SOCKS5 代理：`socks5://127.0.0.1:1080`
- 带认证：`http://username:password@proxy.example.com:8080`

## 📚 高级用法

```bash
# 保存输出到文件（非常适合 AI 分析！）
gh analyze facebook/react -o output.json

# 按文件扩展名过滤搜索
gh search facebook/react useEffect -e .js

# 不区分大小写搜索
gh search facebook/react types --ignore-case

# 绕过缓存并重新克隆
gh analyze facebook/react --no-cache

# 获取更深的目录结构
gh structure facebook/react --depth 4
```

## 🔧 配置

### 缓存位置

仓库缓存在 `~/.gh-cli-cache/`：

```bash
# 清除特定仓库缓存
rm -rf ~/.gh-cli-cache/facebook/react

# 清除所有缓存
rm -rf ~/.gh-cli-cache/
```

### 输出位置

使用 `-o` 选项时，分析结果保存在 `~/.gh-cli-output/`。

## 🤝 贡献

欢迎贡献！请随时提交 Pull Request。

## 📝 许可证

本项目采用 MIT 许可证 - 详见 [LICENSE](LICENSE) 文件。

## 🙏 致谢

- 使用 [commander](https://github.com/tj/commander.js) 构建
- 创建目的是节省 API 配额使用，提供无限制的仓库分析
- 灵感来源于对免费、私密、无限制的 GitHub 仓库探索工具的需求

---

<div align="center">

**由开源社区用 ❤️ 制作**

**厌倦了 API 配额限制？** ⚡ 使用 gh-repo-cli + 你最爱的 AI 助手！

</div>
