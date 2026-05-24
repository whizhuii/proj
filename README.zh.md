# proj

不是项目管理，是项目目录管理。

用分类、模糊搜索和一键 cd 来管理你的 coding 项目目录。

<p align="center">
  <img src="logo.svg" alt="proj" width="160">
</p>

## 这工具是干嘛的？

`~/Project/` 越堆越多——仓库、工具、实验代码、side project。50 个目录之后你就很难：

- **找** — 去年写的那个 Go 工具在哪个目录？→ `proj find tool`
- **跳** — 不想敲完整路径进项目 → `proj blog` 直接进入 `~/Project/blog`
- **看** — 哪些项目还在活跃，哪些在吃灰？→ `proj` 按分类显示一棵树

proj 把你的项目文件夹变成了一个可浏览、可搜索的目录册。它会自动与磁盘保持同步 —— `proj sync` 注册新目录、标记缺失目录，让目录册始终反映真实状态。

## 快速开始

```sh
# 安装二进制（自动检测系统/架构）
curl -fsSL https://raw.githubusercontent.com/whizhuii/proj/main/install.sh | sh

# 添加到 ~/.zshrc 或 ~/.bashrc（eval 方式确保函数始终最新）
eval "$(proj-core shell func)"
eval "$(proj-core shell completion --shell zsh)"   # 或 --shell bash

# 注册已有目录
proj sync

# 查看项目树
proj

# 分类项目（刚同步完都是 uncategorized）
proj mv my-project stable
proj mv old-thing archived
```

<details>
<summary>其他安装方式</summary>

```sh
git clone https://github.com/whizhuii/proj && make build && make install
```
</details>

需要 **git**（`proj clone` / `proj init` 需要）。**fzf** 可选但推荐，用于交互模式。

> `eval "$(proj-core shell func)"` 定义 `proj()` shell 函数，这是必需的——二进制程序无法改变父进程的当前目录，只有 shell 函数能代你执行 `cd`。

## 两种模式

proj 通过 `use_fzf` 设置切换两种控制模式。

### Pass 模式（默认） · `use_fzf: false`

`proj` 显示分类树。输入 `proj <query>` 直接跳转到项目。

![Pass 模式树形列表](demo-pass.gif)

### Fzf 模式 · `use_fzf: true`

`proj` 打开模糊选择器。输入过滤，回车跳转。

![Fzf 模式交互选择](demo-fzf.gif)

## 子命令

除导航外，proj 还提供同步、分类和配置等功能。

### 导航

```
proj                      交互选择器或树形列表
proj blog                 模糊匹配 + cd 到第一个结果
proj -a                   显示全部分类（绕过白名单过滤）
proj go my-project        输出绝对路径（可用于脚本）
```

### 分类

```
proj mv my-project stable      标记为 stable
proj mv old-thing archived     归档
proj rm my-project             移入 removed
proj list                      查看完整树
proj list develop              只看 develop 分类
```

### 克隆或创建

```
proj clone git@github.com:user/repo.git   克隆到 ~/Project/repo + 注册
proj init my-new-thing                    创建目录 + git init + 注册
proj init -t stable my-new-thing          直接创建在 stable 分类下
```

## 命令参考

### 导航

| 命令 | 说明 |
|------|------|
| `proj` | 交互选择器或树形列表（仅白名单分类） |
| `proj <query>` | 模糊匹配项目名并 cd 进入 |
| `proj go <name>` | 输出绝对路径 |
| `proj find <pattern>` | 列出所有匹配的项目 |

### 分类

| 命令 | 说明 |
|------|------|
| `proj list [cat]` | 显示树，可选按分类过滤 |
| `proj mv <name> <cat>` | 移动项目到其他分类 |
| `proj rm <name>` | 移入 removed 分类 |

### 项目创建

| 命令 | 说明 |
|------|------|
| `proj init <name>` | 创建目录 + git init + 注册 |
| `proj clone <url>` | 克隆仓库到项目根 + 注册 |

### 维护

| 命令 | 说明 |
|------|------|
| `proj sync` | 扫描新增/缺失目录 |
| `proj prune` | 删除分类为 `removed` 且磁盘目录已不存在的条目 |
| `proj rename <旧名> <新名>` | 重命名（配置 + 磁盘目录） |
| `proj edit` | 用 $EDITOR 编辑配置 |
| `proj config` | 查看当前配置 |

> **环境变量：** `PROJ_CORE` 可覆盖二进制名称（默认：`proj-core`）。用于测试或自定义构建路径。

## 配置

首次运行自动生成 `~/.config/proj/config.yaml`。以下是完整的配置字段：

```yaml
# 项目在磁盘上的存放路径（默认 ~/Project）。
# 所有 clone/init/sync 操作都发生在此目录下。
project_dir: ~/Project

# 默认 `proj` 树形视图中显示哪些分类。
# 不在列表中的分类会被隐藏，除非加 `-a` 参数。
visible_categories: [develop, stable, uncategorized]

# 设为 true 后，裸 `proj` 始终进入 fzf 交互模式。
# false（默认）时，`proj` 显示分类树形列表。
use_fzf: false

# `proj rm <name>` 的目标分类（默认：removed）。
rm_to: removed

# `proj init <name>` 的目标分类（默认：develop）。
init_to: develop

# `proj clone <url>` 的目标分类（默认：uncategorized）。
clone_to: uncategorized

# `proj sync` 发现新目录时分配的分类（默认：uncategorized）。
sync_new_to: uncategorized

# `proj sync` 发现磁盘上已消失的目录时分配的分类（默认：removed）。
sync_missing_to: removed
```

### 默认分类路由

以上路由字段控制 `proj` 如何自动分类项目：

| 操作/事件 | 默认分类 | 可通过配置项 |
|-----------|----------|-------------|
| `proj init` | `develop` | `init_to` |
| `proj clone` | `uncategorized` | `clone_to` |
| `proj rm` | `removed` | `rm_to` |
| `proj sync` — 发现新目录 | `uncategorized` | `sync_new_to` |
| `proj sync` — 目录已消失 | `removed` | `sync_missing_to` |

### 旧版迁移

旧版本使用 `cat_visible` 映射字段。如果存在，`proj` 会在读取时自动迁移为新的 `visible_categories` 列表。

## 数据存储

proj 在 `~/.config/proj/` 下维护两个文件：

| 文件 | 格式 | 用途 |
|------|------|------|
| `config.yaml` | YAML（键值对） | 配置项 — 项目目录、可见分类、路由默认值 |
| `projects.yaml` | YAML（扁平映射） | 项目注册表 — `项目名: 分类` 条目，每行一条 |

**projects 文件**是分类的事实来源。`proj sync` 将其与 `project_dir`（默认 `~/Project/`）下的实际目录进行比对，根据配置的路由规则添加新条目、标记缺失条目。`proj edit` 命令直接打开此文件供批量编辑。

## Credits

灵感来自 [pass](https://www.passwordstore.org/)，标准的 Unix 密码管理器。

[fzf](https://github.com/junegunn/fzf) — 通用命令行模糊搜索工具 — 为 fzf 模式提供交互式选择器。

由 [DeepSeek V4 Flash](https://deepseek.com) 与 [pro](https://github.com/whizhuii) 共同构建。

## 许可证

MIT
