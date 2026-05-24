# proj

不是项目管理，是项目目录管理。

用分类、模糊搜索和一键 cd 来管理你的 coding 项目目录。

![logo](logo.svg)

![proj 演示](demo.gif)

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
```

<details>
<summary>其他安装方式</summary>

```sh
git clone https://github.com/whizhuii/proj && make build && make install
```
</details>

需要 **git**（`proj clone` / `proj init` 需要）。**fzf** 可选但推荐，用于交互模式。

## 日常使用

### 导航

```
proj                      交互式 fzf 选择器 — 输入过滤，回车跳转
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
| `proj prune` | 清理所有 removed 条目 |
| `proj rename <旧名> <新名>` | 重命名（配置 + 磁盘目录） |
| `proj edit` | 用 $EDITOR 编辑配置 |
| `proj config` | 查看当前配置 |

## 配置

首次运行自动生成 `~/.config/proj/config.yaml`。日常只需关注三个字段：

```yaml
# 项目存放路径（默认 ~/Project）。
project_dir: ~/Project

# 默认树形视图中显示哪些分类。
visible_categories: [develop, stable, uncategorized]

# 设为 true 后，裸 proj 始终进入 fzf 交互模式。
use_fzf: false
```

### 默认分类路由

| 操作 | 分类 |
|------|------|
| `proj init` | `develop` |
| `proj clone` | `uncategorized` |
| `proj rm` | `removed` |
| `proj sync` 新目录 | `uncategorized` |
| `proj sync` 缺失目录 | `removed` |

## Credits

灵感来自 [pass](https://www.passwordstore.org/)，标准的 Unix 密码管理器。

## 许可证

MIT
