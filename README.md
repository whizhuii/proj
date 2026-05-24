# proj

Not project management. Project directory management.

Manage your coding project directories with categories, fuzzy search, and one-key cd.

<p align="center">
  <img src="logo.svg" alt="proj" width="160">
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
```

<details>
<summary>Alternative install methods</summary>

```sh
git clone https://github.com/whizhuii/proj && make build && make install
```
</details>

Requires **git** (for `proj clone` / `proj init`). **fzf** is optional but recommended for interactive mode.

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

![Subcommands: sync go mv config](demo-cmd.gif)

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
| `proj prune` | Remove all `removed` entries |
| `proj rename <old> <new>` | Rename (config + disk directory) |
| `proj edit` | Edit config with `$EDITOR` |
| `proj config` | Show current configuration |

## Configuration

Auto-generated at `~/.config/proj/config.yaml` on first run. Only three fields matter day-to-day:

```yaml
# Where projects live (default: ~/Project).
project_dir: ~/Project

# Which categories show up in the default tree view.
visible_categories: [develop, stable, uncategorized]

# Set true for always-on fzf interactive mode on bare `proj`.
use_fzf: false
```

### Default routing

| Action | Category |
|--------|----------|
| `proj init` | `develop` |
| `proj clone` | `uncategorized` |
| `proj rm` | `removed` |
| `proj sync` new dirs | `uncategorized` |
| `proj sync` missing dirs | `removed` |

## Credits

Inspired by [pass](https://www.passwordstore.org/), the standard Unix password manager.

## License

MIT
