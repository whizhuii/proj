<p align="center">
  <img src="demo/logo.svg" alt="proj" width="160">
</p>

<p align="center">
  管理项目。找到项目。一键跳转。
</p>

<p align="center"><a href="README.md">English</a> | 中文</p>

---

**Fzf 模式**

![Fzf 模式演示](demo/demo-fzf.gif)

**Pass 模式**

![Pass 模式演示](demo/demo-pass.gif)

```
  $ proj                   →   模糊选择器（fzf 模式）
  $ proj <分类>/<项目>     →   按名称跳转（pass 模式）
  $ proj mv <项目> <分类>  →   打标签
  $ proj sync              →   扫描新增/缺失目录
```

---

## 30 秒上手

```sh
curl -fsSL https://raw.githubusercontent.com/whizhuii/proj/master/install.sh | sh
```

然后：

```sh
proj sync                        # 注册已有目录
proj blog                        # 跳进 ~/Project/blog（或在 fzf 里打字选）
proj mv blog stable              # 打上 stable 标签，目录不动
```

## 工作原理

proj 分两层：

- **proj-core**（Rust 二进制）— 管理 YAML 目录、执行 git 操作、输出路径
- **proj()**（shell 函数）— 捕获输出，执行 `cd` 跳转到项目

需要两层的原因：子进程无法改变父 shell 的当前目录——只有和你终端同进程的 shell 函数能执行 `cd`。

你的项目文件平铺在 `~/Project/` 下。一个 YAML 文件（`projects.yaml`）记录每个项目对应的分类标签。目录位置不变，变的只是标签。

## 两种使用方式

### 1. Fzf 模式（默认，推荐）

需要 [fzf](https://github.com/junegunn/fzf)。安装脚本会询问你是否启用。

```
$ proj
```

打字过滤，回车跳转。不需要记全名——输个片段就够了。

### 2. Pass 模式（降级方案，零依赖）

不需要 fzf，任何 shell 都能用。

```
proj <分类>                      → 查看某分类下的项目列表
proj <分类>/<项目>               → 按名称跳转
proj go <项目>                   → 输出绝对路径
proj find <关键字>               → 按名称搜索项目
```

## 你能做什么

proj 的命令操作的是**项目名**和**分类名**——不是目录路径。YAML 目录是事实来源。

### 组织分类

| 命令 | 说明 |
|------|------|
| `proj mv <项目> <分类>` | 移动项目到某分类 |
| `proj mvt <分类> <项目1> <项目2> ...` | 批量移动 |
| `proj rm <项目>` | 标记为 removed（目录保留在磁盘上）|
| `proj list [分类]` | 显示树形列表，可选按分类过滤 |
| `proj rename <旧名> <新名>` | 重命名项目（目录 + 目录一起改）|
| `proj sync` | 扫描新增/缺失目录，更新目录 |
| `proj edit` | 用 `$EDITOR` 批量编辑目录 |

### 跳转

| 命令 | 说明 |
|------|------|
| `proj` | 打开 fzf 选择器（pass 模式显示树形列表）|
| `proj <分类>/<项目>` | 按名称跳转（pass 模式）|
| `proj go <项目>` | 输出项目绝对路径 |
| `proj find <关键字>` | 搜索项目名 |

### 创建

| 命令 | 说明 |
|------|------|
| `proj init <项目名>` | 创建目录 + `git init` + 注册 |
| `proj init -t <分类> <项目名>` | 创建并指定分类 |
| `proj clone <url>` | 克隆仓库到项目根 + 注册 |

### 维护

| 命令 | 说明 |
|------|------|
| `proj prune` | 清理标记为 `removed` 且磁盘目录已不存在的条目 |
| `proj config` | 查看当前配置 |

## 定位对比

```
zoxide/autojump   →  根据使用频率跳转
ghq               →  按来源组织仓库
proj              →  按分类标签组织项目
```

proj 的核心理念是**逻辑组织和目录布局分离**。文件平铺不变，标签负责分类。

## 配置

首次运行自动生成 `~/.config/proj/config.yaml`。

```yaml
# 交互模式：true = fzf 选择器（推荐），false = pass 模式
# 安装脚本会询问你这个设置。
use_fzf: true

# 树形列表中默认显示哪些分类
visible_categories: [develop, stable, uncategorized]

# 项目在磁盘上的存放路径
project_dir: ~/Project

# proj init 的默认分类
init_to: develop
```

> 以上是常用配置子集。完整配置见 `proj-core config --example`。

## 安全

proj **从不删除你的文件**。它只管理 YAML 目录。

- `proj rm <项目>` — 将项目在目录中标记为 `removed`。目录保留在磁盘上。
- `proj prune` — 删除目录中分类为 `removed` **且**磁盘目录已不存在的条目。不会触碰任何现有目录。

**如需从磁盘彻底删除项目：**

```sh
proj rm my-project              # 在目录中标记为 removed
rm -rf ~/Project/my-project     # 从磁盘删除
proj prune                      # 清理目录条目
```

## Credits

- [pass](https://www.passwordstore.org/) — 标准 Unix 密码管理器，proj 双模式设计的灵感来源
- [fzf](https://github.com/junegunn/fzf) — 命令行模糊搜索工具，为交互选择器提供支持
- [opencode](https://opencode.ai) — 构建此项目的 AI 编码助手
- [DeepSeek-V4-Flash](https://chat.deepseek.com/) 与 [DeepSeek-V4-Pro](https://chat.deepseek.com/) — AI 背后的语言模型

## 许可证

MIT
