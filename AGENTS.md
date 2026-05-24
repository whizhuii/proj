# proj — organize your code projects with categories, fuzzy search, and one-key cd

## 构建与安装

```sh
make build        # cargo build --release
make install      # cp + strip → ~/.local/bin/proj-core
make uninstall
make zim-install  # ZIM 补全 → ~/.zim/modules/completion/functions/_proj
```

改代码后：`make install`（自带 strip）。

`.zshrc` / `.bashrc` 集成：
```sh
eval "$(proj-core shell func)"                   # 定义 proj()（bash/zsh 通用）
eval "$(proj-core shell completion --shell zsh)" # zsh 补全
eval "$(proj-core shell completion --shell bash)" # bash 补全
# 或：make zim-install（仅 zsh）
```

## 架构要点

- 单一 Rust crate，`src/main.rs` 单文件。无 linter。
- 依赖：`clap` (derive)、`colored`、`serde` + `serde_yaml`、`dirs`。
- 配置：`~/.config/proj/config.yaml`（首次读取自动生成默认值）。
  - 默认字段：`rm_to: removed`、`init_to: develop`、`clone_to: uncategorized`、`sync_new_to: uncategorized`、`sync_missing_to: removed`、`visible_categories: [develop, stable, uncategorized]`、`project_dir: ~/Project`
  - 旧字段 `cat_visible` 自动迁移为 `visible_categories`
  - `proj`（无参数）默认加 `--filtered` 只显示白名单；`proj -a` 全量
- 项目源数据：`~/.config/proj/projects.yaml`，格式为 `name: category` 的扁平映射。
- 项目目录：由 Config `project_dir` 决定（默认 `~/Project`）。
- Shell 函数 `proj()` 和补全函数（zsh + bash）**以 Rust 字符串常量内嵌在 `src/main.rs` 中**，通过 `proj-core shell func`/`completion` 输出。
  - `shell func` — bash/zsh 通用
  - `shell completion` — 默认 zsh，`--shell bash` 输出 bash 补全
- `proj()` 封装 `proj-core list --flat` + `fzf`，默认受白名单约束，`-a` 绕过滤。
- 二进制本身**从不 `cd`**（子进程无法改变父进程 cwd），由 shell 函数捕获该副作用。
- 测试：`#[cfg(test)]` 内嵌在 `src/main.rs`，`cargo test` 运行（15 个用例）。
- CI：GitHub Actions，ubuntu + macOS 构建 + 测试。

## 关键细节

- 分类为自由字符串，非硬编码。常规：`stable`、`develop`、`dormant`、`archived`、`uncategorized`、`removed`。
- `sync` 跳过 `~/Project/` 中的点号目录。
- `find` 使用**大小写不敏感**的子串匹配（`name.to_lowercase().contains(&query.to_lowercase())`）。
- `edit` 打开临时文件；保存后通过 `serde_yaml` 重新解析为 `BTreeMap`，**顺序不保留**（按 key 重新排序）。
- `prune` 只删除分类为 `rm_to`（默认 `removed`）**且**磁盘目录已不存在的条目。
- `proj()` 尊重 `PROJ_CORE` 环境变量（默认 `proj-core`）。
- `repo_name_from_url()` 同时处理 `git@host:user/repo` 和 `https://host/user/repo.git`。
- `proj` 的模糊匹配通过 `fzf` 实现，匹配目标为 `cat/proj` 拼接行。
- `proj <query>` 非交互式 fzf 过滤，`proj -a <query>` 全量过滤。
- `proj <Tab>` 列出命令 + 项目名补全（仅白名单）。
