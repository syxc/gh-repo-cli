# Claude Code 集成指南

> ghr 与 Claude Code 结合使用的最佳实践

## 核心理念

**让 AI 主动判断何时使用工具，而不是手动调用。**

将 ghr 的使用规则添加到你的 `~/.claude/CLAUDE.md` 全局配置文件中，Claude Code 会自动检测何时需要分析 GitHub 仓库。

---

## 快速配置

### 1. 安装 ghr

**方式 A: 使用 go install（推荐）**
```bash
go install github.com/syxc/gh-repo-cli@latest
```

**方式 B: 下载二进制文件**
```bash
# macOS/Linux
curl -L -o ghr "https://github.com/syxc/gh-repo-cli/releases/latest/download/ghr-$(uname -s)-$(uname -m)"
chmod +x ghr
sudo mv ghr /usr/local/bin/
```

**方式 C: 使用 npm（遗留）**
```bash
npm install -g @oknian1/gh-repo-cli
```

### 2. 添加配置到 CLAUDE.md

编辑 `~/.claude/CLAUDE.md` 文件，添加以下内容：

```
# GitHub 仓库分析优先级

MCP (zread): 有限配额 ❌
ghr: 完全免费 ✅

触发规则（用户输入匹配 → 使用 ghr）:
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

MCP 备用条件（仅在以下情况使用 MCP）:
1. 私有仓库（ghr 仅支持公开仓库）
2. 需要 git 历史
3. 用户明确要求使用 MCP
```

### 3. 开始使用

```
你: "我想了解 Next.js 的项目架构"
Claude Code: [自动运行 ghr analyze vercel/next.js 并分析]
```

---

## 对比优势

| 方面 | MCP 服务器 | ghr |
|------|-----------|-----|
| **使用配额** | 100-500 次/月 ❌ | 无限 ✅ |
| **速率限制** | 60 请求/小时 ❌ | 无限制 ✅ |
| **费用** | $10-50/月 ❌ | 完全免费 ✅ |
| **隐私** | 代码发送到第三方 ❌ | 本地分析 ✅ |
| **可靠性** | 依赖服务器 ❌ | 离线工作 ✅ |
| **速度** | 依赖网络 ⚠️ | 本地缓存 ⚡ |

---

## 使用示例

### 场景 1: 探索新项目架构

```
你: "我想看看 TypeScript 是如何组织的"

Claude Code:
  $ ghr structure microsoft/TypeScript --depth 2

  TypeScript 项目的目录结构如下：
  ├── src/          # 编译器核心代码
  ├── scripts/      # 构建脚本
  └── tests/        # 测试文件
```

### 场景 2: 查找功能实现

```
你: "Vite 是如何实现 HMR 的？"

Claude Code:
  $ ghr search vitejs/vite "HMR|hot.*module"
  $ ghr read vitejs/vite packages/vite/src/server/moduleGraph.ts

  Vite 的 HMR 实现在 moduleGraph.ts 中，通过 WebSocket 连接...
```

### 场景 3: 学习配置规范

```
你: "ESLint 的配置文件有哪些格式？"

Claude Code:
  $ ghr search eslint/eslint "config.*file" -e .md
  $ ghr read eslint/eslint docs/use/configuration/README.md

  ESLint 支持 .eslintrc.js、.eslintrc.json、.eslintrc.yml 等格式...
```

### 场景 4: 对比项目差异

```
你: "Webpack 和 Vite 的构建方式有什么不同？"

Claude Code:
  $ ghr analyze webpack/webpack
  $ ghr analyze vitejs/vite

  两者核心区别：
  - Webpack 使用打包模式，Vite 使用 ESM + 原生浏览器
  - Vite 启动速度更快，开发体验更好...
```

---

## 最佳实践

### 自然提问，让 AI 主导

✅ **正确**: "Tailwind CSS 的架构是怎样的？" → Claude 自动调用 ghr  
❌ **错误**: "运行 ghr analyze tailwindlabs/tailwindcss"

### 由宽到深，逐步深入

```
推荐流程:
  1. "这个项目的整体结构是什么？" → ghr structure
  2. "路由功能在哪里实现？" → ghr search
  3. "给我看看路由器的核心代码" → ghr read
```

### 充分利用缓存

```bash
# 首次分析：克隆仓库（10-30 秒）
ghr analyze vercel/next.js

# 后续查询：直接使用缓存（<1 秒）
ghr search vercel/next.js "router"
ghr read vercel/next.js packages/next/server/router.ts

# 如需更新：强制刷新
ghr analyze vercel/next.js --no-cache
```

### 保存分析结果

```bash
# 保存为 JSON 供 AI 处理
ghr analyze facebook/react -o react-analysis.json

# 然后在对话中引用：
# "@react-analysis.json 解释代码库架构"
```

---

## 故障排查

### Claude 没有使用 ghr

**检查配置**:
```bash
cat ~/.claude/CLAUDE.md
```

应包含 "GitHub 仓库分析优先级" 部分

### ghr 命令未找到

**检查安装**:
```bash
# 如果使用 go install
export PATH=$PATH:$(go env GOPATH)/bin

# 验证安装
which ghr
ghr --version
```

**如需重新安装**:
```bash
go install github.com/syxc/gh-repo-cli@latest
```

### Claude 使用 MCP 而不是 ghr

这是**预期行为**！以下情况会自动降级到 MCP：
1. 私有仓库（ghr 仅支持公开仓库）
2. 需要 git 历史记录
3. 用户明确要求使用 MCP

---

## 总结

**配置一次，永久生效**：
1. 安装 ghr（1 分钟）
2. 添加配置到 CLAUDE.md（10 秒）
3. 自然对话即可使用（零学习）

**核心优势**：
- ✅ AI 自动检测，无需手动调用
- ✅ 无限使用，完全免费
- ✅ 本地分析，隐私安全
- ✅ 智能降级，兼容私有仓库
- ✅ Go 编写，快速可靠

---

<div align="center">

**CLAUDE.md 指令 + ghr = 自动仓库分析** 🚀

</div>
