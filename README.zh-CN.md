# ghr (gh-repo-cli)

> 轻量级 GitHub 仓库分析工具，无需 API Token

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.85%2B-orange)](https://www.rust-lang.org)
[![CI](https://github.com/syxc/gh-repo-cli/workflows/CI/badge.svg)](https://github.com/syxc/gh-repo-cli/actions)
[![Code Quality](https://github.com/syxc/gh-repo-cli/workflows/Code%20Quality/badge.svg)](https://github.com/syxc/gh-repo-cli/actions)

ghr 用 `git` 克隆仓库并在本地完成分析：语言统计、文件类型、目录结构、代码搜索、文件读取。适合终端下直接使用，也适合需要仓库上下文的 AI 编码助手，避免消耗 API 或 MCP 配额。仅支持公开仓库。

## 安装

从 [Releases](https://github.com/syxc/gh-repo-cli/releases) 页面下载对应平台的压缩包，解压出 `ghr` 二进制：

```bash
# macOS ARM64；请替换 <version> 与平台（见 releases 页面）
curl -LO "https://github.com/syxc/gh-repo-cli/releases/latest/download/ghr_<version>_darwin_arm64.tar.gz"
tar -xzf ghr_<version>_darwin_arm64.tar.gz
sudo mv ghr /usr/local/bin/
```

可用平台：`linux_amd64`、`linux_arm64`、`darwin_amd64`、`darwin_arm64`、`windows_amd64`（`.zip`），SHA256 见 `checksums.txt`。

或从源码构建（需要 Rust 1.85+ 与 Git）：

```bash
git clone https://github.com/syxc/gh-repo-cli.git
cd gh-repo-cli
cargo install --path .
```

## 使用

```bash
ghr analyze facebook/react              # 语言、文件类型、结构
ghr structure vuejs/core --depth 2      # 目录树（深度默认 3）
ghr search facebook/react useState -e .js
ghr search facebook/react types --ignore-case
ghr read facebook/react README.md
ghr ls facebook/react src               # 深度默认 1
ghr readme facebook/react

ghr clean facebook/react                # 删除单个缓存仓库
ghr clean --all                         # 清空缓存
ghr clean --old 30d                     # 清理超过 30 天未使用的仓库
ghr clean                               # 显示用法与缓存统计
```

需要注意的行为：

- `search` 接受正则表达式。表格最多显示 50 条；`-o` 始终保存完整结果。
- `analyze`、`structure`、`search` 支持 `-o <file>` 输出 JSON（自动创建目录）。
- `--no-cache` 重新克隆，而不是更新缓存副本。
- `read` 输出文件内容；文件不存在时会列出其父目录内容，便于找到正确路径。

## 代理

代理在每次调用时重新解析，系统代理变更后下一条命令即自动生效。按顺序取第一个非空值：

1. `GH_PROXY`
2. `HTTPS_PROXY` / `https_proxy`
3. `HTTP_PROXY` / `http_proxy`
4. `ALL_PROXY` / `all_proxy`
5. 系统设置：macOS 读 `scutil --proxy`，Linux 读 GNOME `gsettings`，Windows 读注册表

```bash
GH_PROXY="http://127.0.0.1:7890" ghr analyze facebook/react   # 临时覆盖
```

## 缓存与输出

仓库缓存在 `~/.ghr-cache/<owner>/<repo>`，`ghr clean` 用于删除其中的条目。`~/.ghr-output/` 仅被 `ghr clean -o` 使用（整体删除）；`-o` 本身始终写入你指定的路径。

长期使用的缓存不会失控膨胀：

- 克隆是浅克隆且只取默认分支的最新提交（`--depth 1 --single-branch`）；每次刷新会过期 reflog 并修剪被替换的对象，缓存仓库不会随时间增大。
- LRU 式保留策略：每次使用仓库都会刷新其"最近使用"标记。`ghr clean --old 30d` 逐出超过 30 天未使用的仓库；裸 `ghr clean` 会显示缓存大小与最旧条目作为提醒。
- 自动逐出：设置 `GH_CACHE_MAX_AGE`（格式相同，如 `30d`），ghr 在每次调用访问缓存前自动清理过期仓库：

  ```bash
  export GH_CACHE_MAX_AGE=30d
  ```

## 文档

- [AI 集成指南](docs/AI_INTEGRATION.zh-CN.md) — 在 Claude Code 等助手中使用 ghr
- [发布工作流](docs/RELEASE_WORKFLOW.md) — 版本标签如何变成一次发布
- [测试指南](docs/TESTING.md) — 运行与编写测试
- [贡献指南](CONTRIBUTING.md) — 开发环境与 PR 流程

## 许可证

MIT — 见 [LICENSE](LICENSE)。
