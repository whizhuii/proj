<p align="center">
  <img src="logo.svg" alt="proj" width="160">
</p>

<h1 align="center">proj</h1>

<p align="center">
  Not project management. Project directory management.<br>
  Manage your coding project directories with categories, fuzzy search, and one-key cd.
</p>

## What is proj?

Your `~/Project/` fills up fast — repos, tools, experiments, side projects. Soon you have 50+ directories and no easy way to:

- **Find** where that Go tool you wrote last year lives → `proj find tool`
- **Jump** into a project without typing the full path → `proj blog` drops you in `~/Project/blog`
- **See** what's active vs gathering dust → `proj` shows a tree grouped by category

proj turns your project folder into a browsable, searchable catalog. It keeps itself in sync with your disk — `proj sync` registers new directories and flags missing ones, so the catalog always matches reality.

## Quick start

```sh
# Install binary (auto-detects OS/arch)
curl -fsSL https://raw.githubusercontent.com/whizhuii/proj/main/install.sh | sh

# Add to ~/.zshrc or ~/.bashrc (eval lines keep the function up-to-date)
eval "$(proj-core shell func)"
eval "$(proj-core shell completion --shell zsh)"   # or --shell bash

# Register existing directories
proj sync

# See your project tree
proj

# Categorize projects (all start as uncategorized)
proj mv my-project stable
proj mv old-thing archived
```

<details>
<summary>Alternative install methods</summary>

```sh
git clone https://github.com/whizhuii/proj && make build && make install
```
</details>

Requires **git** (for `proj clone` / `proj init`). **fzf** is optional but recommended for interactive mode.

> The `eval "$(proj-core shell func)"` line defines the `proj()` shell function, which is required because a binary cannot change the parent shell's working directory — only the function can `cd` on your behalf.

## Two modes

proj has two control modes toggled by the `use_fzf` setting.

### Pass mode (default) · `use_fzf: false`

`proj` shows a categorized tree. Type `proj <query>` to jump directly to a project.

![Pass mode tree view](demo-pass.gif)

### Fzf mode · `use_fzf: true`

`proj` opens a fuzzy picker. Type to filter, press Enter to cd.

![Fzf mode interactive picker](demo-fzf.gif)

## Subcommands

Beyond navigation, proj handles synchronization, categorization, and configuration.

### Navigate

```
proj                      Interactive picker or tree view
proj blog                 Fuzzy match + cd into first result
proj -a                   Show all categories (bypass visibility filter)
proj go my-project        Print the absolute path (useful in scripts)
```

### Categorize

```
proj mv my-project stable     Tag it as stable
proj mv old-thing archived    Archive it
proj rm my-project            Move to removed
proj list                     See the full tree
proj list develop             See only develop projects
```

### Clone or init

```
proj clone git@github.com:user/repo.git   # Clone into ~/Project/repo + register
proj init my-new-thing                     # mkdir + git init + register
proj init -t stable my-new-thing           # Init directly into stable
```

## Command reference

### Navigation

| Command | Description |
|---------|-------------|
| `proj` | Interactive picker or tree view (filtered by visible categories) |
| `proj <query>` | Fuzzy match against project name and cd |
| `proj go <name>` | Print absolute path |
| `proj find <pattern>` | List all projects matching pattern |

### Categorization

| Command | Description |
|---------|-------------|
| `proj list [cat]` | Show tree, optionally filtered by category |
| `proj mv <name> <cat>` | Move project to a different category |
| `proj rm <name>` | Move project to `removed` |

### Project creation

| Command | Description |
|---------|-------------|
| `proj init <name>` | `mkdir` + `git init` + register |
| `proj clone <url>` | `git clone` into project root + register |

### Maintenance

| Command | Description |
|---------|-------------|
| `proj sync` | Scan for new/missing directories |
| `proj prune` | Remove entries categorized as `removed` whose disk directory no longer exists |
| `proj rename <old> <new>` | Rename (config + disk directory) |
| `proj edit` | Edit config with `$EDITOR` |
| `proj config` | Show current configuration |

> **Environment variable:** `PROJ_CORE` overrides the binary name (default: `proj-core`). Useful for testing or when using a custom build path.

## Configuration

Auto-generated at `~/.config/proj/config.yaml` on first run. Below is the full config with all available fields:

```yaml
# Where projects live on disk (default: ~/Project).
# All clone/init/sync operations happen under this directory.
project_dir: ~/Project

# Which categories show up in the default `proj` tree view.
# Categories not in this list are hidden unless you pass `-a`.
visible_categories: [develop, stable, uncategorized]

# Set true for always-on fzf interactive mode on bare `proj`.
# When false (default), `proj` shows a categorized tree.
use_fzf: false

# Target category for `proj rm <name>` (default: removed).
rm_to: removed

# Target category for `proj init <name>` (default: develop).
init_to: develop

# Target category for `proj clone <url>` (default: uncategorized).
clone_to: uncategorized

# Category assigned to newly discovered directories on `proj sync` (default: uncategorized).
sync_new_to: uncategorized

# Category assigned to directories that vanished from disk on `proj sync` (default: removed).
sync_missing_to: removed
```

### Default routing

The routing fields above control how `proj` auto-classifies projects:

| Action / Event | Default category | Configurable via |
|----------------|------------------|------------------|
| `proj init` | `develop` | `init_to` |
| `proj clone` | `uncategorized` | `clone_to` |
| `proj rm` | `removed` | `rm_to` |
| `proj sync` — new directory found | `uncategorized` | `sync_new_to` |
| `proj sync` — directory missing | `removed` | `sync_missing_to` |

### Legacy migration

Older versions used a `cat_visible` map field. If present, `proj` auto-migrates it to the new `visible_categories` list on read.

## Data storage

proj maintains two files under `~/.config/proj/`:

| File | Format | Purpose |
|------|--------|---------|
| `config.yaml` | YAML (key-value) | Settings — project directory, visible categories, routing defaults |
| `projects.yaml` | YAML (flat map) | Project registry — `project_name: category` entries, one per line |

The **projects file** is the source of truth for categorization. `proj sync` reconciles it against the actual directories under `project_dir` (`~/Project/` by default), adding new entries and marking missing ones according to the configured routing rules. The `proj edit` command opens this file directly for bulk changes.

## Credits

Inspired by [pass](https://www.passwordstore.org/), the standard Unix password manager.

[fzf](https://github.com/junegunn/fzf) — a general-purpose command-line fuzzy finder — powers the interactive picker in fzf mode.

## License

MIT

---

*This project was written with [DeepSeek-V4-Flash](https://chat.deepseek.com/) and [DeepSeek-V4-Pro](https://chat.deepseek.com/).*
