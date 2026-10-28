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
tar -xzf "ghr_<version>_darwin_arm64.tar.gz"
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
ghr analyze facebook/react --depth 3    # 更深的结构树（默认 2）
ghr analyze facebook/react --sections languages   # 只输出语言表
ghr structure vuejs/core --depth 2      # 目录树（深度默认 3）
ghr search facebook/react useState -e .js
ghr search facebook/react types --ignore-case
ghr read facebook/react README.md
ghr ls facebook/react src               # 深度默认 1
ghr readme facebook/react

ghr clean facebook/react                # 删除单个缓存仓库
ghr clean --all                         # 删除可识别的缓存仓库
ghr clean --old 30d                     # 清理超过 30 天未使用的仓库
ghr clean                               # 显示用法与缓存统计
```

需要注意的行为：

- `search` 接受 Rust 正则表达式。表格最多显示 50 条；`-o` 保存全部匹配，无匹配时不写文件。二进制文件（前 8 KiB 含 NUL 字节）会被跳过。
- `analyze` 统计根目录及下一层目录中的文件（深度 2），表格和 JSON 均只保留前 10 种语言和文件类型；`--depth` 只改变结构树。
- `analyze`、`structure`、`search` 支持 `-o <file>` 输出 JSON，并按需创建父目录。`analyze --sections` 可选择 `info`、`languages`、`file-types`、`structure`；JSON 中 `file-types` 对应 `fileTypes`，且始终保留 `repo`。
- `--no-cache` 强制重新克隆。普通刷新失败时返回错误并保留原缓存。
- 命令参数须为有效 UTF-8。仓库内容中的终端控制字符会转义，正常换行和制表符保留。
- `read` 输出文件内容，文件不存在时列出父目录，便于找到正确路径。`read`、`ls`、`search`、`readme` 的文件访问限于仓库内，符号链接的目标也受此限制。

## 代理

代理在每次调用时重新解析，系统代理变更后下一条命令即自动生效。按顺序取第一个非空值：

1. `GH_PROXY`
2. `HTTPS_PROXY` / `https_proxy`
3. `HTTP_PROXY` / `http_proxy`
4. `ALL_PROXY` / `all_proxy`
5. 系统设置：macOS 读 `scutil --proxy`，Linux 读 GNOME `gsettings`，Windows 读注册表。优先 HTTP(S) 代理；仅配置 SOCKS 时以 `socks5h://`（远程 DNS）使用

```bash
GH_PROXY="http://127.0.0.1:7890" ghr analyze facebook/react   # 临时覆盖
```

## 缓存与输出

仓库缓存在 `~/.ghr-cache/<owner>/<repo>`，可用 `GH_CACHE_DIR` 更换位置；已有目录须为空或可识别的 ghr 缓存。缓存根目录和所有者目录中的普通 `.DS_Store` 文件会保留，不影响识别；GitHub 所有者名和仓库名按 ASCII 大小写不敏感方式比较。同一缓存根目录下的命令串行执行，等待锁最多 30 秒。

- 克隆深度为 1。每次刷新以深度 1 获取当前默认分支，不获取标签，再运行 `git gc --auto`，保留 Git 的正常到期策略。缓存大小仍会随仓库内容和保留对象增长。
- 清理只删除已确认的 GitHub 仓库目录，保留缓存根目录和无关文件；拒绝 owner/repo 路径上的符号链接。
- `ghr clean --old 30d` 删除至少 30 天未使用的仓库；裸 `ghr clean` 显示缓存大小与最旧条目。每次成功使用都会刷新最近使用标记。
- 设置 `GH_CACHE_MAX_AGE`（如 `30d`），在克隆或刷新仓库前清理过期仓库，保留当前使用的仓库：

  ```bash
  export GH_CACHE_MAX_AGE=30d
  ```

`-o <file>` 写入指定路径；`ghr clean -o` 删除 `~/.ghr-output/`，不会追踪这些输出文件。

## 文档

- [AI 集成指南](docs/AI_INTEGRATION.zh-CN.md) — 在 Claude Code 等助手中使用 ghr
- [发布工作流](docs/RELEASE_WORKFLOW.md) — 版本标签如何变成一次发布
- [测试指南](docs/TESTING.md) — 运行与编写测试
- [贡献指南](CONTRIBUTING.md) — 开发环境与 PR 流程

## 许可证

MIT — 见 [LICENSE](LICENSE)。
