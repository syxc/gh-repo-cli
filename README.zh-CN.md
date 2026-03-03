# gh-repo-cli

> 轻量级 GitHub 仓库分析工具，无需 API Token

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Node.js Version](https://img.shields.io/node/v/gh-repo-cli)](https://github.com/syxc/gh-repo-cli)
[![CI](https://github.com/syxc/gh-repo-cli/workflows/CI/badge.svg)](https://github.com/syxc/gh-repo-cli/actions)
[![Code Quality](https://github.com/syxc/gh-repo-cli/workflows/Code%20Quality/badge.svg)](https://github.com/syxc/gh-repo-cli/actions)

在终端中分析、搜索和探索 GitHub 仓库 - 无需 API Token。

## 🎯 为什么需要这个工具？

### 问题现状

许多 AI 编码助手（如 GLM Coding Plan、Claude Code、Cursor 等）通过 **MCP（模型上下文协议）** 服务器或内置工具提供 GitHub 仓库分析功能。然而，这些服务通常存在**使用配额限制**，影响你的工作效率：

- **GLM Coding Lite**: 每月有限的 API 调用次数
- **GitHub API**: 速率限制（未认证用户 60 次/小时）
- **MCP 服务器**: 通常有每日/每月配额
- **付费工具**: 重度使用时订阅费用昂贵

### 解决方案

**gh-repo-cli** 是一个**免费、无限制的替代方案**：

- ✅ 使用 `git clone` 而非 GitHub API - **无速率限制**
- ✅ **独立使用**或**与任何 AI 助手配合使用**
- ✅ 本地缓存仓库以实现**快速重新分析**
- ✅ 支持**代理**访问，随时随地访问 GitHub
- ✅ 提供**结构化 JSON 输出**，方便 AI 解析

### 使用场景

#### 1. 独立命令行工具

直接在终端中使用，进行快速仓库分析：

```bash
ghr analyze facebook/react
ghr search vuejs/core ref
ghr read facebook/react README.md
```

#### 2. 与 AI 编码助手配合（推荐！）

**最佳实践**：将 gh-repo-cli 的使用规则添加到 `~/.claude/CLAUDE.md` 全局配置，让 AI 自动检测何时需要分析仓库。

**一键配置** - 在 `~/.claude/CLAUDE.md` 中添加：

```
# GitHub 仓库分析优先级

GLM MCP (zread): 有限配额 ❌
gh-repo-cli: 完全免费 ✅

触发规则（用户输入匹配 → 使用 gh-repo-cli）:
- github.com/ 链接
- "github 仓库" | "分析.*仓库" | "仓库.*分析"
- "查看.*代码" | "阅读.*源码" | "clone.*github"
- owner/repo 格式（如 facebook/react）

使用方式：
ghr analyze <owner/repo>           # 完整分析
ghr search <owner/repo> <query>    # 搜索代码
ghr structure <owner/repo>         # 获取结构
ghr read <owner/repo> <file>       # 读取文件
ghr readme <owner/repo>            # 读取 README

MCP 备用条件（仅在以下情况使用 zread MCP）:
1. 私有仓库（gh-repo-cli 仅支持公开仓库）
2. 需要 git 历史
3. 用户明确要求使用 MCP
```

**配置后直接自然对话**：

```
你: "React 是如何实现 hooks 的？"

Claude Code:
  $ ghr analyze facebook/react
  $ ghr search facebook/react "useState" -e .js
  $ ghr read facebook/react packages/react/src/ReactHooks.js

  基于仓库分析，React hooks 的实现方式是...
```

**优势**：
- ✅ **零配置** - 无需创建 skill 文件
- ✅ **自动检测** - AI 决定何时使用 gh-repo-cli
- ✅ **自然交互** - 用自然语言提问，无需手动调用命令
- ✅ **智能降级** - 私有仓库时自动使用 MCP
- ✅ **始终生效** - 所有对话都可用

📖 **完整指南**：[docs/AI_INTEGRATION.zh-CN.md](docs/AI_INTEGRATION.zh-CN.md) - 详细示例、高级工作流和故障排查

#### 3. 其他 AI 助手

**Cursor / Windsurf / Copilot**:
```bash
# 在终端分析仓库
ghr analyze vuejs/core -o vue-analysis.json

# 在 AI 聊天中引用输出
@vue-analysis.json 解释 Vue 的响应式系统
```

**ChatGPT / Claude (网页版)**:
```bash
# 导出仓库数据
ghr analyze tensorflow/tensorflow -o tf.json

# 上传 JSON 文件并提问
```

#### 4. MCP 与 CLI 对比

| 特性 | MCP 服务器 | gh-repo-cli |
|------|-----------|-------------|
| **使用限制** | ❌ 通常有限制 | ✅ 无限制 |
| **配置** | ⚠️ 配置 Token/服务器 | ✅ 一段 CLAUDE.md 配置 |
| **隐私** | ⚠️ 代码通过第三方服务器 | ✅ 本地分析 |
| **费用** | 💰 付费/配额限制 | ✅ 免费 |
| **速度** | ⚠️ 依赖网络 | ⚡ 本地缓存 |
| **AI 检测** | ❌ 手动调用 | ✅ 自动检测 |

## ✨ 特性

- 🔍 **无需 API Token** - 使用 git clone 而非 GitHub API
- 📊 **全面分析** - 语言检测、文件统计、目录结构
- 🔎 **代码搜索** - 在整个代码库中搜索模式
- 📁 **文件操作** - 读取文件、列出目录
- 🌐 **代理支持** - 支持 HTTP/HTTPS/SOCKS5 代理
- ⚡ **本地缓存** - 仓库本地缓存，快速后续访问
- 🔒 **安全** - 除了 git clone 操作外，数据不离开你的机器
- 🤖 **AI 友好** - JSON 输出格式，易于与 AI 助手集成

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
ghr analyze facebook/react

# 获取目录结构
ghr structure vuejs/core

# 搜索代码模式
ghr search facebook/react useState

# 读取特定文件
ghr read facebook/react README.md

# 列出目录中的文件
ghr ls facebook/react/src

# 清理缓存仓库
ghr clean --all              # 清理所有缓存
ghr clean facebook/react     # 清理特定仓库
```

### AI 辅助工作流（推荐）

```bash
# 步骤 1: 探索仓库
ghr structure facebook/react --depth 2

# 步骤 2: 搜索特定模式
ghr search facebook/react useEffect -e .js -o search_results.json

# 步骤 3: 读取相关文件
ghr read facebook/react packages/react/src/ReactHooks.js

# 步骤 4: 与 AI 助手分享发现
# (Claude Code, Cursor, Copilot 等)
```

## 🌐 代理支持

如果你在防火墙后或需要通过代理访问 GitHub：

```bash
# 设置代理环境变量
export GH_PROXY="http://127.0.0.1:7890"

# 或每个命令单独设置
GH_PROXY="http://127.0.0.1:7890" ghr analyze facebook/react
```

支持的代理类型：
- HTTP/HTTPS 代理: `http://127.0.0.1:7890`
- SOCKS5 代理: `socks5://127.0.0.1:1080`
- 带认证: `http://username:password@proxy.example.com:8080`

## 📚 高级用法

```bash
# 保存输出到文件（非常适合 AI 分析！）
ghr analyze facebook/react -o output.json

# 按文件扩展名过滤搜索
ghr search facebook/react useEffect -e .js

# 不区分大小写搜索
ghr search facebook/react types --ignore-case

# 绕过缓存重新克隆
ghr analyze facebook/react --no-cache

# 获取更深的目录结构
ghr structure facebook/react --depth 4
```

## 🔧 配置

### 缓存位置

仓库缓存在 `~/.ghr-cache/`：

```bash
# 清理特定仓库缓存（使用 CLI）
ghr clean facebook/react

# 清理所有缓存（使用 CLI）
ghr clean --all

# 或手动清理
rm -rf ~/.ghr-cache/facebook/react
rm -rf ~/.ghr-cache/
```

### 输出位置

使用 `-o` 选项时，分析结果保存在 `~/.ghr-output/`。

## 📖 更多文档

- 🤖 **[AI 集成指南](docs/AI_INTEGRATION.zh-CN.md)** - Claude Code 集成最佳实践
- 🚀 **[发布工作流指南](docs/RELEASE_WORKFLOW.md)** - 自动发布配置指南
- 🧪 **[测试指南](docs/TESTING.md)** - 测试指南

## 🤝 贡献

欢迎贡献！请随时提交 Pull Request。

## 📝 许可证

本项目采用 MIT 许可证 - 详见 [LICENSE](LICENSE) 文件。

## 🙏 致谢

- 使用 [commander](https://github.com/tj/commander.js) 构建
- 旨在节省 API 配额使用，提供无限制仓库分析
- 灵感来源于对免费、私密、无限制 GitHub 仓库探索的需求

---

<div align="center">

**由开源社区用 ❤️ 制作**

**厌倦了 API 配额？** ⚡ 使用 gh-repo-cli + 你喜欢的 AI 助手！

</div>
