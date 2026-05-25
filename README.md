<p align="center">
  <img src="demo/logo.svg" alt="proj" width="160">
</p>

<p align="center">
  Organize your projects. Find them. Jump in one key.
</p>

---

**Fzf mode**

![Fzf mode demo](demo/demo-fzf.gif)

**Pass-like mode**

![Pass mode demo](demo/demo-pass.gif)

```
  $ proj                   →   fuzzy picker (fzf mode)
  $ proj <cat>/<project>   →   jump by name (pass-like mode)
  $ proj mv <name> <cat>   →   tag a project
  $ proj sync              →   scan for new/missing directories
```

---

## In 30 seconds

```sh
curl -fsSL https://raw.githubusercontent.com/whizhuii/proj/master/install.sh | sh
```

Then:

```sh
proj sync                        # register existing directories
proj blog                        # jump into ~/Project/blog (or type in fzf picker)
proj mv blog stable              # tag it, directory stays put
```

## How it works

proj is two pieces:

- **proj-core** (Rust binary) — manages the YAML catalog, runs git operations, prints paths
- **proj()** (shell function) — captures the output and `cd`s you into the project

Two layers because a child process cannot change its parent shell's working directory — only the shell function running in your terminal can.

Your projects stay flat in `~/Project/`. A YAML catalog (`projects.yaml`) maps each one to a category tag. The directory never moves — only the tag changes.

## Two ways to use proj

### 1. Fzf mode (default, recommended)

Requires [fzf](https://github.com/junegunn/fzf). The install script will ask if you want to enable it.

```
$ proj
```

Type to filter, press Enter to `cd`. No need to remember exact names — just type a fragment.

### 2. Pass-like mode (fallback, zero dependencies)

No fzf required. Works with any shell.

```
proj <cat>                      → list projects in a category
proj <cat>/<project>            → jump into the project
proj go <project>               → print the absolute path
proj find <pattern>             → search projects by name
```

## What you can do

proj commands work with **project names** and **category names** — never with directory paths. The catalog is the source of truth.

### Organize

| Command | Description |
|---------|-------------|
| `proj mv <name> <cat>` | Move a project to a category |
| `proj mvt <cat> <names...>` | Batch move |
| `proj rm <name>` | Mark as removed (directory stays on disk) |
| `proj list [cat]` | Show tree, optionally filtered by category |
| `proj rename <old> <new>` | Rename project (catalog + disk directory) |
| `proj sync` | Scan for new/missing directories, update catalog |
| `proj edit` | Bulk edit the catalog with `$EDITOR` |

### Jump

| Command | Description |
|---------|-------------|
| `proj` | Open fzf picker (or show tree in pass mode) |
| `proj <cat>/<project>` | Jump into a project by name (pass mode) |
| `proj go <name>` | Print the absolute path |
| `proj find <pattern>` | Search projects matching a pattern |

### Create

| Command | Description |
|---------|-------------|
| `proj init <name>` | Create directory + `git init` + register |
| `proj init -t <cat> <name>` | Init directly into a category |
| `proj clone <url>` | `git clone` into project root + register |

### Maintain

| Command | Description |
|---------|-------------|
| `proj prune` | Remove catalog entries tagged as `removed` whose disk directory no longer exists |
| `proj config` | Show current configuration |

## How it compares

```
zoxide/autojump   →  freqency-based jump
ghq               →  organize by source host
proj              →  organize by category tag
```

proj separates **logical organization** from **directory layout**. Your files stay flat; the tag does the organizing.

## Configuration

Auto-generated at `~/.config/proj/config.yaml`.

```yaml
# Interactive mode: true = fzf picker (recommended), false = pass mode
# The install script asks about this.
use_fzf: true

# Which categories appear by default in the tree view
visible_categories: [develop, stable, uncategorized]

# Where projects live on disk
project_dir: ~/Project

# Default category for proj init
init_to: develop
```

> Subset shown. See `proj-core config --example` for the full reference.

## Safety

proj **never deletes your files**. It only manages the YAML catalog.

- `proj rm <name>` — re-categorizes the project to `removed`. The directory stays on disk.
- `proj prune` — removes catalog entries that are already categorized as `removed` **and** whose disk directory no longer exists. Never touches existing directories.

**To actually delete a project:**

```sh
proj rm my-project              # mark as removed in catalog
rm -rf ~/Project/my-project     # delete from disk
proj prune                      # clean up catalog entry
```

## Credits

- [pass](https://www.passwordstore.org/) — the standard Unix password manager, which inspired proj's two-mode design
- [fzf](https://github.com/junegunn/fzf) — the fuzzy finder powering the interactive picker
- [opencode](https://opencode.ai) — the AI coding assistant that built this project
- [DeepSeek-V4-Flash](https://chat.deepseek.com/) & [DeepSeek-V4-Pro](https://chat.deepseek.com/) — the language models behind the AI

## License

MIT
