use clap::{Parser, Subcommand};
use colored::*;
use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::PathBuf;
use std::process::Command;

type Projects = BTreeMap<String, String>;

fn display_path(path: &PathBuf) -> String {
    if let Some(home) = dirs::home_dir() {
        if let Ok(rest) = path.strip_prefix(&home) {
            return format!("~/{}{}", rest.display(), if path.is_dir() { "/" } else { "" });
        }
    }
    path.display().to_string()
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    rm_to: String,
    init_to: String,
    clone_to: String,
    sync_new_to: String,
    sync_missing_to: String,
    #[serde(default = "default_visible_categories")]
    visible_categories: Vec<String>,
    #[serde(default)]
    use_fzf: bool,
    #[serde(default)]
    no_git: bool,
    #[serde(default = "default_project_dir")]
    project_dir: String,
}

fn default_project_dir() -> String {
    "~/Project".to_string()
}

fn expand_tilde(path: &PathBuf) -> PathBuf {
    if let Some(p) = path.to_str() {
        if p.starts_with("~/") {
            if let Some(home) = dirs::home_dir() {
                return home.join(&p[2..]);
            }
        }
        if p == "~" {
            if let Some(home) = dirs::home_dir() {
                return home;
            }
        }
    }
    path.clone()
}

fn default_visible_categories() -> Vec<String> {
    vec!["develop".into(), "stable".into(), "uncategorized".into()]
}

fn default_categories() -> Vec<String> {
    vec![
        "develop".into(),
        "stable".into(),
        "uncategorized".into(),
        "dormant".into(),
        "archived".into(),
        "removed".into(),
    ]
}

impl Default for Config {
    fn default() -> Self {
        Self {
            rm_to: "removed".to_string(),
            init_to: "develop".to_string(),
            clone_to: "uncategorized".to_string(),
            sync_new_to: "uncategorized".to_string(),
            sync_missing_to: "removed".to_string(),
            visible_categories: default_visible_categories(),
            use_fzf: false,
            no_git: false,
            project_dir: default_project_dir(),
        }
    }
}

impl Config {
    fn is_category_visible(&self, cat: &str) -> bool {
        self.visible_categories.is_empty()
            || self.visible_categories.iter().any(|c| c == cat)
    }
}

fn settings_path() -> PathBuf {
    let base = dirs::config_dir().unwrap_or_else(|| {
        let home = dirs::home_dir().expect("Cannot find home directory");
        home.join(".config")
    });
    base.join("proj").join("config.yaml")
}

fn read_settings() -> Config {
    let path = settings_path();
    if path.exists() {
        let content = fs::read_to_string(&path).unwrap_or_default();
        let mut root: Value = serde_yaml::from_str(&content).unwrap_or_default();

        let mut migrated = false;

        // Migrate old cat_visible → visible_categories
        if let Some(old) = root.get("cat_visible") {
            if let Some(map) = old.as_mapping() {
                let visible: Vec<String> = map
                    .iter()
                    .filter(|(_, v)| v.as_bool() == Some(true))
                    .map(|(k, _)| k.as_str().unwrap_or("").to_string())
                    .collect();
                if !visible.is_empty() {
                    root["visible_categories"] = Value::Sequence(
                        visible.into_iter().map(Value::String).collect(),
                    );
                }
            }
            if let Some(m) = root.as_mapping_mut() {
                m.remove("cat_visible");
            }
            migrated = true;
        }

        // Ensure visible_categories exists; if not, write default
        if root.get("visible_categories").is_none() {
            root["visible_categories"] = Value::Sequence(
                default_visible_categories()
                    .into_iter()
                    .map(Value::String)
                    .collect(),
            );
            migrated = true;
        }

        let config: Config = serde_yaml::from_value(root).unwrap_or_default();

        if migrated {
            if let Some(parent) = path.parent() {
                let _ = fs::create_dir_all(parent);
            }
            let yaml = serde_yaml::to_string(&config).expect("Failed to serialize migrated config");
            let _ = fs::write(&path, yaml);
        }

        let example_path = path.with_file_name("config.example.yaml");
        if !example_path.exists() {
            let _ = fs::write(&example_path, CONFIG_EXAMPLE);
        }

        config
    } else {
        let config = Config::default();
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let yaml = serde_yaml::to_string(&config).expect("Failed to serialize config");
        let _ = fs::write(&path, yaml);
        let example_path = path.with_file_name("config.example.yaml");
        let _ = fs::write(&example_path, CONFIG_EXAMPLE);
        config
    }
}

fn projects_path() -> PathBuf {
    let base = dirs::config_dir().unwrap_or_else(|| {
        let home = dirs::home_dir().expect("Cannot find home directory");
        home.join(".config")
    });
    base.join("proj").join("projects.yaml")
}

fn project_root() -> PathBuf {
    let settings = read_settings();
    let path = expand_tilde(&PathBuf::from(&settings.project_dir));
    if !path.exists() {
        fs::create_dir_all(&path).expect("Failed to create project directory");
    }
    path
}

fn read_projects() -> Projects {
    let path = projects_path();
    if !path.exists() {
        return Projects::new();
    }
    let content = fs::read_to_string(&path).unwrap_or_default();
    serde_yaml::from_str(&content).unwrap_or_default()
}

fn write_projects(projects: &Projects) {
    let path = projects_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("Failed to create config directory");
    }
    let content = serde_yaml::to_string(projects).expect("Failed to serialize");
    fs::write(&path, content).expect("Failed to write config");
}

fn categories_path() -> PathBuf {
    let base = dirs::config_dir().unwrap_or_else(|| {
        let home = dirs::home_dir().expect("Cannot find home directory");
        home.join(".config")
    });
    base.join("proj").join("categories.yaml")
}

fn read_categories() -> Vec<String> {
    let path = categories_path();
    if !path.exists() {
        let cats = default_categories();
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let yaml = serde_yaml::to_string(&cats).expect("Failed to serialize categories");
        let _ = fs::write(&path, yaml);
        return cats;
    }
    let content = fs::read_to_string(&path).unwrap_or_default();
    serde_yaml::from_str(&content).unwrap_or_default()
}

fn repo_name_from_url(url: &str) -> String {
    url.trim_end_matches(".git")
        .trim_end_matches('/')
        .split('/')
        .last()
        .unwrap_or(url)
        .to_string()
}

fn format_sorted(projects: &Projects) -> String {
    let mut entries: Vec<(&String, &String)> = projects.iter().collect();
    entries.sort_by(|a, b| a.1.cmp(b.1).then(a.0.cmp(b.0)));
    let mut s = String::new();
    for (name, cat) in &entries {
        s.push_str(&format!("{}: {}\n", name, cat));
    }
    s
}

#[derive(Parser)]
#[command(name = "proj-core", about = "Project directory manager backend")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Scan ~/Project/ for new/missing directories and update config
    Sync,
    /// List projects grouped by category
    #[command(alias = "ls")]
    List {
        #[arg(long, help = "Disable colored output")]
        no_color: bool,
        #[arg(long = "cats", short = 'c', help = "Show only category names with counts")]
        cats: bool,
        #[arg(long, help = "Print only project names (one per line, for shell completion)")]
        names: bool,
        #[arg(long, help = "Print category/project one per line (for shell completion)")]
        flat: bool,
        #[arg(short = 'a', long = "all", help = "Show all categories (ignore visibility rules and empty filter)")]
        all: bool,
        #[arg(long, help = "Only show categories in visible_categories (pass category arg to override)")]
        filtered: bool,
        #[arg(help = "Filter by category (stable, develop, dormant, archived, deleted, ...)")]
        category: Option<String>,
    },
    /// Move a project to a different category
    Mv {
        name: String,
        category: String,
    },
    /// Rename a project (updates config + disk directory)
    Rename {
        name: String,
        new_name: String,
    },
    /// Move one or more projects to a category
    Mvt {
        category: String,
        names: Vec<String>,
    },
    /// Clone a git repository and add to config
    Clone {
        url: String,
        #[arg(long, help = "Category to assign")]
        to: Option<String>,
        #[arg(trailing_var_arg = true, allow_hyphen_values = true, num_args = .., help = "Extra arguments passed to git clone")]
        git_args: Vec<String>,
    },
    /// Print the path to a project's directory
    Go {
        name: String,
    },
    /// Edit config with $EDITOR (sorted by category, then name)
    Edit,
    /// Move one or more projects to deleted category
    Rm {
        names: Vec<String>,
    },
    /// Remove all 'deleted' entries from config
    Prune,
    /// Create a new project directory with git init
    Init {
        name: String,
        #[arg(short = 't', long = "to", help = "Category to assign")]
        to: Option<String>,
    },
    /// Search projects matching a pattern
    Find {
        query: Vec<String>,
    },
    /// Show or write config
    Config {
        #[arg(long, help = "Output as YAML")]
        yaml: bool,
        #[arg(long, help = "Print annotated config example")]
        example: bool,
        #[arg(long, help = "Check if fzf mode is enabled (returns true/false)")]
        fzf: bool,
        #[arg(long, help = "Print project root directory")]
        project_dir: bool,
    },
    /// List all known categories
    Categories,
    /// Output shell function for eval
    Shell {
        #[command(subcommand)]
        command: ShellCommand,
    },
}
                        
#[derive(Subcommand)]
enum ShellCommand {
    /// Output shell function (bash/zsh compatible)
    Func,
    /// Output completion (--mode zsh/zim/bash, default: zsh)
    Completion {
        #[arg(long, value_enum, default_value_t = ShellMode::Zsh)]
        mode: ShellMode,
    },
}

#[derive(clap::ValueEnum, Clone)]
enum ShellMode {
    /// zsh eval mode (ends with compdef _proj proj)
    Zsh,
    /// ZIM autoload mode (ends with _proj)
    Zim,
    /// bash mode
    Bash,
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Sync => cmd_sync(),
        Commands::List {
            no_color,
            names,
            flat,
            cats,
            all,
            filtered,
            category,
        } => cmd_list(no_color, names, flat, cats, all, filtered, category.as_deref()),
        Commands::Find { query } => cmd_find(&query.join(" ")),
        Commands::Mv { name, category } => cmd_mv(&name, &category),
        Commands::Rename { name, new_name } => cmd_rename(&name, &new_name),
        Commands::Mvt { category, names } => cmd_mvt(&category, &names),
        Commands::Clone { url, to, git_args } => cmd_clone(&url, to.as_deref(), &git_args),
        Commands::Go { name } => cmd_go(&name),
        Commands::Edit => cmd_edit(),
        Commands::Rm { names } => cmd_rm(&names),
        Commands::Prune => cmd_prune(),
        Commands::Init { name, to } => cmd_init(&name, to.as_deref()),
        Commands::Config { yaml, example, fzf, project_dir } => cmd_config(yaml, example, fzf, project_dir),
        Commands::Categories => cmd_categories(),
        Commands::Shell { command } => match command {
            ShellCommand::Func => cmd_shell_func(),
            ShellCommand::Completion { mode } => cmd_shell_completion(mode),
        },
    }
}

fn git_available() -> bool {
    Command::new("git").arg("--version").output().is_ok()
}

fn try_git_init(dir: &std::path::Path, no_git: bool) {
    if no_git {
        return;
    }
    if !git_available() {
        eprintln!("Warning: git not found, skipping git init");
        return;
    }
    let status = Command::new("git")
        .args(["init", &dir.to_string_lossy()])
        .status()
        .expect("Failed to run git");
    if !status.success() {
        eprintln!("Warning: git init failed");
    }
}

fn cmd_sync() {
    let settings = read_settings();
    let root = project_root();
    let mut projects = read_projects();

    let mut disk_dirs: Vec<String> = Vec::new();
    let entries = match fs::read_dir(&root) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("Error: cannot read {}: {}", display_path(&root), e);
            std::process::exit(1);
        }
    };

    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };
        let name = entry.file_name().to_string_lossy().to_string();
        if name.starts_with('.') {
            continue;
        }
        if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
            disk_dirs.push(name);
        }
    }

    let mut added = 0;
    for name in &disk_dirs {
        if !projects.contains_key(name) {
            projects.insert(name.clone(), settings.sync_new_to.clone());
            added += 1;
        }
    }

    let mut missing = 0;
    let keys: Vec<String> = projects.keys().cloned().collect();
    for name in &keys {
        if projects.get(name) == Some(&settings.rm_to) {
            continue;
        }
        if !disk_dirs.contains(name) {
            projects.insert(name.clone(), settings.sync_missing_to.clone());
            missing += 1;
        }
    }

    write_projects(&projects);
    println!(
        "✓ Synced: {} added, {} missing, {} total",
        added,
        missing,
        projects.len()
    );
}

fn cmd_list(no_color: bool, names: bool, flat: bool, cats: bool, all: bool, _filtered: bool, filter: Option<&str>) {
    let settings = read_settings();
    let projects = read_projects();
    let root = project_root();

    if projects.is_empty() {
        let has_dirs = fs::read_dir(&root)
            .map(|d| d.filter_map(|e| e.ok()).any(|e| e.file_type().map(|t| t.is_dir()).unwrap_or(false)))
            .unwrap_or(false);
        if has_dirs {
            println!("No registered projects. Run 'proj sync' to register existing directories.");
        } else {
            println!("{} is empty.", display_path(&root));
            println!("Create a project:  proj init <name>");
            println!("Clone a project:   proj clone <url>");
        }
        return;
    }

    let cat_allowed = |cat: &str| -> bool {
        if all {
            true
        } else if let Some(f) = filter {
            cat == f
        } else {
            settings.is_category_visible(cat)
        }
    };

    if names {
        let mut entries: Vec<&String> = projects.keys().collect();
        entries.sort();
        for name in entries {
            let cat = projects.get(name).map(|c| c.as_str()).unwrap_or("");
            if !cat_allowed(cat) {
                continue;
            }
            println!("{}", name);
        }
        return;
    }

    if flat {
        let mut entries: Vec<(&String, &String)> = projects.iter().collect();
        entries.sort_by(|a, b| a.1.cmp(b.1).then(a.0.cmp(b.0)));
        for (name, cat) in entries {
            if !cat_allowed(cat.as_str()) {
                continue;
            }
            println!("{}/{}", cat, name);
        }
        return;
    }

    let mut by_cat: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for (name, cat) in &projects {
        if !cat_allowed(cat.as_str()) {
            continue;
        }
        by_cat.entry(cat.clone()).or_default().push(name.clone());
    }

    for names in by_cat.values_mut() {
        names.sort();
    }

    if all {
        let known_cats: BTreeSet<&str> = [
            settings.rm_to.as_str(),
            settings.init_to.as_str(),
            settings.clone_to.as_str(),
            settings.sync_new_to.as_str(),
            settings.sync_missing_to.as_str(),
        ]
        .into_iter()
        .chain(projects.values().map(|s| s.as_str()))
        .collect();
        for c in known_cats {
            if let Some(f) = filter {
                if c != f {
                    continue;
                }
            }
            by_cat.entry(c.to_string()).or_default();
        }
    } else {
        for cat in &settings.visible_categories {
            by_cat.entry(cat.clone()).or_default();
        }
        by_cat.retain(|cat, _| settings.is_category_visible(cat));
        if let Some(f) = filter {
            by_cat.retain(|cat, _| cat == f);
        }
    }

    if by_cat.is_empty() {
        if let Some(f) = filter {
            println!("No projects in category '{}'", f);
        }
        return;
    }

    let tree_root = filter.unwrap_or("proj");
    let cat_entries: Vec<(&String, &Vec<String>)> = by_cat.iter().collect();
    let n_cats = cat_entries.len();

    if n_cats == 1 && (cats || filter.is_some()) {
        let (cat, names) = &cat_entries[0];
        if no_color {
            if cats {
                println!("{} ({})", cat, names.len());
            } else {
                println!("{}", cat);
                for (j, name) in names.iter().enumerate() {
                    let pfx = if j == names.len().saturating_sub(1) { "└── " } else { "├── " };
                    let path = root.join(name);
                    if !path.exists() {
                        println!("{}{}  (missing)", pfx, name);
                    } else {
                        println!("{}{}", pfx, name);
                    }
                }
            }
        } else {
            if cats {
                println!("{} ({})", cat.to_string().color(Color::Blue).bold(), names.len());
            } else {
                println!("{}", cat.to_string().color(Color::Blue).bold());
                for (j, name) in names.iter().enumerate() {
                    let pfx = if j == names.len().saturating_sub(1) { "└── " } else { "├── " };
                    let path = root.join(name);
                    if !path.exists() {
                        println!("{}{}  {}", pfx, name, "(missing)".dimmed());
                    } else {
                        println!("{}{}", pfx, name);
                    }
                }
            }
        }
        return;
    }

    if no_color {
        println!("{}", tree_root);
        for (i, (cat, names)) in cat_entries.iter().enumerate() {
            let is_last = i == n_cats - 1;
            let cat_pfx = if is_last { "└── " } else { "├── " };
            let child_pfx = if is_last { "    " } else { "│   " };

            if cats {
                println!("{}{} ({})", cat_pfx, cat, names.len());
            } else {
                println!("{}{}", cat_pfx, cat);
            }

            if !cats {
                for (j, name) in names.iter().enumerate() {
                    let pfx = if j == names.len().saturating_sub(1) { "└── " } else { "├── " };
                    let path = root.join(name);
                    if !path.exists() {
                        println!("{}{}{}  (missing)", child_pfx, pfx, name);
                    } else {
                        println!("{}{}{}", child_pfx, pfx, name);
                    }
                }
            }
        }
    } else {
        println!("{}", tree_root);
        for (i, (cat, names)) in cat_entries.iter().enumerate() {
            let is_last = i == n_cats - 1;
            let cat_pfx = if is_last { "└── " } else { "├── " };
            let child_pfx = if is_last { "    " } else { "│   " };

            if cats {
                println!("{}{} ({})", cat_pfx, cat.to_string().color(Color::Blue).bold(), names.len());
            } else {
                println!("{}{}", cat_pfx, cat.to_string().color(Color::Blue).bold());
            }

            if !cats {
                for (j, name) in names.iter().enumerate() {
                    let pfx = if j == names.len().saturating_sub(1) { "└── " } else { "├── " };
                    let path = root.join(name);
                    if !path.exists() {
                        println!("{}{}{}  {}", child_pfx, pfx, name, "(missing)".dimmed());
                    } else {
                        println!("{}{}{}", child_pfx, pfx, name);
                    }
                }
            }
        }
    }
}

fn cmd_mv(name: &str, category: &str) {
    let settings = read_settings();
    let mut projects = read_projects();

    if !projects.contains_key(name) {
        eprintln!("Error: project '{}' not found in config", name);
        std::process::exit(1);
    }

    if !read_categories().contains(&category.to_string()) {
        eprintln!(
            "Error: '{}' is not a registered category.",
            category,
        );
        eprintln!("Registered categories: {}", known_categories().into_iter().collect::<Vec<_>>().join(", "));
        std::process::exit(1);
    }

    if category != settings.rm_to {
        let dir_path = project_root().join(name);
        if !dir_path.exists() {
            eprintln!(
                "Error: directory '{}' does not exist on disk (use '{}' category instead)",
                dir_path.display(),
                settings.rm_to,
            );
            std::process::exit(1);
        }
    }

    projects.insert(name.to_string(), category.to_string());
    write_projects(&projects);
    println!("✓ '{}' moved to {}", name, category);
}

fn cmd_rename(name: &str, new_name: &str) {
    let mut projects = read_projects();

    if !projects.contains_key(name) {
        eprintln!("Error: project '{}' not found in config", name);
        std::process::exit(1);
    }
    if projects.contains_key(new_name) {
        eprintln!("Error: project '{}' already exists in config", new_name);
        std::process::exit(1);
    }

    let root = project_root();
    let old_path = root.join(name);
    let new_path = root.join(new_name);

    if old_path.exists() {
        if new_path.exists() {
            eprintln!("Error: '{}' already exists on disk", new_path.display());
            std::process::exit(1);
        }
        fs::rename(&old_path, &new_path).expect("Failed to rename directory");
    }

    let cat = projects.remove(name).unwrap();
    projects.insert(new_name.to_string(), cat);
    write_projects(&projects);

    println!("✓ Renamed '{}' → '{}'", name, new_name);
}

fn cmd_mvt(category: &str, names: &[String]) {
    let settings = read_settings();
    let mut projects = read_projects();
    let mut moved = 0;

    let cats = read_categories();
    if !cats.contains(&category.to_string()) {
        eprintln!("Error: '{}' is not a registered category.", category);
        eprintln!("Registered categories: {}", cats.join(", "));
        std::process::exit(1);
    }

    for name in names {
        if !projects.contains_key(name) {
            eprintln!("Error: project '{}' not found in config", name);
            continue;
        }
        if category != settings.rm_to {
            let dir_path = project_root().join(name);
            if !dir_path.exists() {
                eprintln!(
                    "Error: directory '{}' does not exist on disk (use '{}' category instead)",
                    dir_path.display(),
                    settings.rm_to,
                );
                continue;
            }
        }
        projects.insert(name.clone(), category.to_string());
        moved += 1;
    }
    write_projects(&projects);
    println!("✓ Moved {} project(s) to {}", moved, category);
}

fn cmd_clone(url: &str, to: Option<&str>, git_args: &[String]) {
    let settings = read_settings();
    let to = to.unwrap_or(&settings.clone_to);

    let cats = read_categories();
    if !cats.contains(&to.to_string()) {
        eprintln!("Error: '{}' is not a registered category.", to);
        eprintln!("Registered categories: {}", cats.join(", "));
        std::process::exit(1);
    }

    let name = repo_name_from_url(url);
    let dest = project_root().join(&name);

    if dest.exists() {
        eprintln!("Error: '{}' already exists", dest.display());
        std::process::exit(1);
    }

    if settings.no_git || !git_available() {
        if !settings.no_git {
            eprintln!("Warning: git not found, creating empty directory instead of cloning");
        }
        fs::create_dir_all(&dest).expect("Failed to create project directory");
    } else {
        let mut cmd = Command::new("git");
        cmd.arg("clone");
        cmd.args(git_args.iter().map(|s| s.as_str()));
        cmd.arg(url);
        cmd.arg(dest.to_string_lossy().as_ref());
        let status = cmd.status().expect("Failed to run git");

        if !status.success() {
            eprintln!("Error: git clone failed");
            std::process::exit(1);
        }
    }

    let mut projects = read_projects();
    projects.insert(name.clone(), to.to_string());
    write_projects(&projects);
    println!("✓ Cloned '{}' as {}", name, to);
}

fn cmd_go(name: &str) {
    let projects = read_projects();
    if !projects.contains_key(name) {
        eprintln!("Error: project '{}' not found in config", name);
        std::process::exit(1);
    }
    let path = project_root().join(name);
    if !path.exists() {
        eprintln!(
            "Error: directory '{}' does not exist on disk",
            path.display()
        );
        std::process::exit(1);
    }
    println!("{}", path.display());
}

fn cmd_edit() {
    let projects = read_projects();
    let editor = std::env::var("EDITOR").unwrap_or_else(|_| "vi".to_string());

    let content = format_sorted(&projects);
    let tmp_path = std::env::temp_dir().join("proj-edit.yaml");
    fs::write(&tmp_path, &content).expect("Failed to write temp file");

    let status = Command::new(&editor)
        .arg(&tmp_path)
        .status()
        .expect("Failed to launch editor");

    if !status.success() {
        eprintln!("Editor exited with error");
        let _ = fs::remove_file(&tmp_path);
        std::process::exit(1);
    }

    let new_content = fs::read_to_string(&tmp_path).unwrap_or_default();
    let _ = fs::remove_file(&tmp_path);

    let new_projects: Projects = match serde_yaml::from_str(&new_content) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error: invalid YAML: {}", e);
            std::process::exit(1);
        }
    };

    // Validate all categories are registered
    let cats = read_categories();
    for (name, cat) in &new_projects {
        if !cats.contains(cat) {
            eprintln!("Error: project '{}' has unregistered category '{}'.", name, cat);
            eprintln!("Registered categories: {}", cats.join(", "));
            std::process::exit(1);
        }
    }

    write_projects(&new_projects);
    println!("✓ Config updated ({} projects)", new_projects.len());
}

fn cmd_rm(names: &[String]) {
    let settings = read_settings();
    let mut projects = read_projects();
    let mut moved = 0;
    for name in names {
        if !projects.contains_key(name) {
            eprintln!("Error: project '{}' not found in config", name);
            continue;
        }
        projects.insert(name.clone(), settings.rm_to.clone());
        moved += 1;
    }
    write_projects(&projects);
    println!("✓ Moved {} project(s) to {}", moved, settings.rm_to);
}

fn cmd_prune() {
    let settings = read_settings();
    let root = project_root();
    let mut projects = read_projects();
    let to_prune: Vec<String> = projects
        .iter()
        .filter(|(name, cat)| **cat == settings.rm_to && !root.join(name).exists())
        .map(|(name, _)| name.clone())
        .collect();

    for name in &to_prune {
        projects.remove(name);
    }

    let pruned = to_prune.len();
    if pruned == 0 {
        println!("No {} entries to prune (dir must be gone)", settings.rm_to);
        return;
    }
    write_projects(&projects);
    println!("✓ Pruned {} {} entries ({} remaining)", pruned, settings.rm_to, projects.len());
}

fn cmd_init(name: &str, to: Option<&str>) {
    let settings = read_settings();
    let to = to.unwrap_or(&settings.init_to);

    let cats = read_categories();
    if !cats.contains(&to.to_string()) {
        eprintln!("Error: '{}' is not a registered category.", to);
        eprintln!("Registered categories: {}", cats.join(", "));
        std::process::exit(1);
    }

    let dest = project_root().join(name);

    if dest.exists() {
        eprintln!("Error: '{}' already exists", dest.display());
        std::process::exit(1);
    }

    fs::create_dir_all(&dest).expect("Failed to create project directory");

    try_git_init(&dest, settings.no_git);

    let mut projects = read_projects();
    projects.insert(name.to_string(), to.to_string());
    write_projects(&projects);
    println!("✓ Created '{}' as {}", name, to);
}

fn cmd_find(query: &str) {
    let projects = read_projects();
    let root = project_root();

    let mut by_cat: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for (name, cat) in &projects {
        if name.to_lowercase().contains(&query.to_lowercase()) {
            by_cat.entry(cat.clone()).or_default().push(name.clone());
        }
    }

    for names in by_cat.values_mut() {
        names.sort();
    }

    if by_cat.is_empty() {
        println!("Search Terms: {}\n  No results found.", query);
        return;
    }

    let cat_entries: Vec<(&String, &Vec<String>)> = by_cat.iter().collect();
    let n_cats = cat_entries.len();

    println!("Search Terms: {}", query);
    for (i, (cat, names)) in cat_entries.iter().enumerate() {
        let is_last = i == n_cats - 1;
        let cat_pfx = if is_last { "└── " } else { "├── " };
        let child_pfx = if is_last { "    " } else { "│   " };

        println!("{}{}", cat_pfx, cat.to_string().color(Color::Blue).bold());

        for (j, name) in names.iter().enumerate() {
            let pfx = if j == names.len().saturating_sub(1) { "└── " } else { "├── " };
            let path = root.join(name);
            if !path.exists() {
                println!("{}{}{}  {}", child_pfx, pfx, name, "(missing)".dimmed());
            } else {
                println!("{}{}{}", child_pfx, pfx, name);
            }
        }
    }
}

fn known_categories() -> BTreeSet<String> {
    read_categories().into_iter().collect()
}

fn cmd_config(yaml: bool, example: bool, fzf: bool, project_dir: bool) {
    let settings = read_settings();
    if fzf {
        println!("{}", if settings.use_fzf { "true" } else { "false" });
        return;
    }
    if project_dir {
        println!("{}", display_path(&project_root()));
        return;
    }
    if example {
        print!("{}", CONFIG_EXAMPLE);
        return;
    }
    if yaml {
        println!("{}", serde_yaml::to_string(&settings).unwrap());
    } else {
        println!("rm_to:           {}", settings.rm_to);
        println!("init_to:         {}", settings.init_to);
        println!("clone_to:        {}", settings.clone_to);
        println!("sync_new_to:     {}", settings.sync_new_to);
        println!("sync_missing_to: {}", settings.sync_missing_to);
        println!("use_fzf:         {}", settings.use_fzf);
        println!("no_git:          {}", settings.no_git);
        println!("project_dir:     {}", settings.project_dir);
        if !settings.visible_categories.is_empty() {
            println!("visible_categories: [{}]", settings.visible_categories.join(", "));
        }
        let cats = read_categories();
        if !cats.is_empty() {
            println!("categories:        [{}]", cats.join(", "));
        }
    }
}

fn cmd_categories() {
    let cats = known_categories();
    for c in cats {
        println!("{}", c);
    }
}

const CONFIG_EXAMPLE: &str = r#"# ============================================================
# proj configuration file
# Location: ~/.config/proj/config.yaml
#
# Auto-generated on first run. Edits take effect immediately — no restart needed.
# ============================================================

# --- Category routing ---
# These fields set the default category for each operation.
# Category names must be registered in categories.yaml.
# Common values: stable / develop / dormant / archived / removed / uncategorized.

# rm_to: category for proj rm <project> (deletion target)
rm_to: removed

# init_to: default category for proj init <name>
init_to: develop

# clone_to: default category for proj clone <url>
clone_to: uncategorized

# sync_new_to: category assigned to newly discovered directories on sync
sync_new_to: uncategorized

# sync_missing_to: category assigned when a directory disappears from disk
sync_missing_to: removed

# --- Interactive mode ---
# Set true to always use fzf interactive picker on bare `proj`.
# Falls back to tree view if fzf is not installed.
use_fzf: false

# --- Git ---
# Set true to skip all git operations (git init on `proj init`, git clone on `proj clone`).
# When true, `proj clone` creates an empty directory instead of cloning.
# Default: false (git is used for all operations).
# no_git: true

# --- Project root ---
# Directory where all projects live. Default: ~/Project.
# project_dir: ~/Project

# --- Visibility filter (optional) ---
# visible_categories lists which categories appear in the default tree view.
# Empty list = show all (no filtering).
# Use proj -a / --all to bypass the filter.
#
# Example:
# visible_categories:
#   - develop
#   - stable
#   - uncategorized
visible_categories:
  - develop
  - stable
  - uncategorized
"#;

const PROJ_SHELL: &str = r#"
proj() {
  local PROJ_CORE="${PROJ_CORE:-proj-core}"

  if [[ $# -eq 0 ]]; then
    if [[ "$($PROJ_CORE config --fzf 2>/dev/null)" == "true" ]]; then
      if ! command -v fzf >/dev/null 2>&1; then
        echo "proj: warning: use_fzf enabled but fzf not found; falling back to pass style" >&2
        "$PROJ_CORE" list --filtered
        return
      fi
      local selected
      selected="$($PROJ_CORE list --flat --filtered \
        | fzf --height=60% --layout=reverse --info=hidden --prompt='proj> ' --padding=1)" || return 1
      local dir
      dir="$($PROJ_CORE go "${selected#*/}" 2>/dev/null)" && cd "$dir"
      return
    fi
    "$PROJ_CORE" list --filtered
    return
  fi

  case "$1" in
    help|--help|-h)
      echo "Usage: proj [options] [command|query]"
      echo ""
      echo "Modes:"
      echo "  proj                    Show project tree (pass-style, filtered)"
      echo "  proj -a                 Show all projects (bypass visibility filter)"
      echo "  proj <query>            List category or cd to project (cat/proj)"
      echo "  proj -a <query>         List category or cd to project (all categories)"
      echo ""
      echo "Commands:"
      printf "  %-9s %s\n" "sync"    "Scan project root for new/missing directories"
      printf "  %-9s %s\n" "list"    "List projects grouped by category"
      printf "  %-9s %s\n" "find"    "Search projects matching a pattern"
      printf "  %-9s %s\n" "go"      "Print path to a project directory"
      printf "  %-9s %s\n" "mv"      "Move a project to a different category"
      printf "  %-9s %s\n" "mvt"     "Move multiple projects to a category"
      printf "  %-9s %s\n" "rename"  "Rename a project (config + disk directory)"
      printf "  %-9s %s\n" "clone"   "Clone a git repository and add to config"
      printf "  %-9s %s\n" "init"    "Create a new project directory with git init"
      printf "  %-9s %s\n" "edit"    "Edit config with \$EDITOR"
      printf "  %-9s %s\n" "rm"      "Move projects to 'removed' category"
      printf "  %-9s %s\n" "prune"   "Remove all 'removed' entries from config"
      printf "  %-9s %s\n" "config"  "Show or write config"
      echo ""
      echo "Options:"
      echo "  -a, --all   Show all categories (bypass visibility filter)"
      echo "  -h, --help  Print this help"
      echo ""
      echo "Config: ~/.config/proj/config.yaml"
      echo "  Set use_fzf: true for fzf interactive mode on bare 'proj'"
      echo "  See 'proj-core config --example' for full reference"
      ;;
    -a|--all)
      shift
      local q="$*"
      if [[ -n "$q" ]]; then
        if [[ "$($PROJ_CORE config --fzf 2>/dev/null)" == "true" ]]; then
          if ! command -v fzf >/dev/null 2>&1; then
            echo "proj: warning: use_fzf enabled but fzf not found; falling back to pass style" >&2
            "$PROJ_CORE" list "${1%/}"
            return
          fi
          local flat_cmd="$PROJ_CORE list --flat"
          local selected
          selected="$(eval "$flat_cmd" | fzf --filter="$q" | head -1)" || {
            echo "proj: no match for '$q'" >&2
            return 1
          }
          local dir
          dir="$($PROJ_CORE go "${selected#*/}" 2>/dev/null)" && cd "$dir"
          return
        fi

        # pass mode: strip trailing /, check cat/proj pattern
        local arg="${1%/}"
        if [[ "$arg" == */* ]]; then
          local dir
          dir="$($PROJ_CORE go "${arg#*/}" 2>/dev/null)" && cd "$dir"
          return
        fi
        "$PROJ_CORE" list "$arg"
        return
      fi
      if [[ "$($PROJ_CORE config --fzf 2>/dev/null)" == "true" ]]; then
        if ! command -v fzf >/dev/null 2>&1; then
          echo "proj: warning: use_fzf enabled but fzf not found; falling back to pass style" >&2
          "$PROJ_CORE" list --all
          return
        fi
        local selected
        selected="$($PROJ_CORE list --flat --all | fzf --height=60% --layout=reverse --info=hidden --prompt='proj> ' --padding=1)" || return 1
        local dir
        dir="$($PROJ_CORE go "${selected#*/}" 2>/dev/null)" && cd "$dir"
        return
      fi
      "$PROJ_CORE" list --all
      ;;
    sync|list|find|clone|go|edit|config|prune|init|mv|mvt|rename|rm)
      $PROJ_CORE "$@"
      ;;
    *)
      if [[ "$($PROJ_CORE config --fzf 2>/dev/null)" == "true" ]]; then
        if ! command -v fzf >/dev/null 2>&1; then
          echo "proj: warning: use_fzf enabled but fzf not found; falling back to pass style" >&2
          # pass fallback: strip trailing /, check cat/proj
          local arg="${1%/}"
          if [[ "$arg" == */* ]]; then
            local dir
            dir="$($PROJ_CORE go "${arg#*/}" 2>/dev/null)" && cd "$dir"
            return
          fi
          "$PROJ_CORE" list "$arg"
          return
        fi
        local q="$*"
        local selected
        selected="$($PROJ_CORE list --flat --filtered | fzf --filter="$q" | head -1)" || {
          echo "proj: no match for '$q'" >&2
          return 1
        }
        local dir
        dir="$($PROJ_CORE go "${selected#*/}" 2>/dev/null)" && cd "$dir"
        return
      fi

      # pass mode
      local arg="${1%/}"
      if [[ "$arg" == */* ]]; then
        local dir
        dir="$($PROJ_CORE go "${arg#*/}" 2>/dev/null)" && cd "$dir"
        return
      fi
      "$PROJ_CORE" list "$arg"
      ;;
  esac
}"#;

const PROJ_ZSH_COMPLETION: &str = r#"#compdef proj

_proj() {
  local cur="${words[$CURRENT]}"
  local -a projs cats all

  if (( CURRENT > 2 )); then
    (( CURRENT-- ))
    shift words
    local cmd="$words[1]"

    case "$cmd" in
      sync|find|edit|config|prune) ;;
      list|ls)
        cats=("${(@f)$(proj-core categories 2>/dev/null)}")
        (( $#cats )) && _describe 'category' cats
        ;;
      go|rm)
        projs=("${(@f)$(proj-core list --names --all 2>/dev/null)}")
        (( $#projs )) && _describe 'project' projs
        ;;
      mv)
        if (( CURRENT == 2 )); then
          projs=("${(@f)$(proj-core list --names --all 2>/dev/null)}")
          (( $#projs )) && _describe 'project' projs
        else
          cats=("${(@f)$(proj-core categories 2>/dev/null)}")
          (( $#cats )) && _describe 'category' cats
        fi
        ;;
      mvt)
        if (( CURRENT == 2 )); then
          cats=("${(@f)$(proj-core categories 2>/dev/null)}")
          (( $#cats )) && _describe 'category' cats
        else
          projs=("${(@f)$(proj-core list --names --all 2>/dev/null)}")
          (( $#projs )) && _describe 'project' projs
        fi
        ;;
      rename)
        if (( CURRENT == 2 )); then
          projs=("${(@f)$(proj-core list --names --all 2>/dev/null)}")
          (( $#projs )) && _describe 'project' projs
        fi
        ;;
      clone|init)
        if [[ "$words[$((CURRENT-1))]" == (--to|-t) ]]; then
          cats=("${(@f)$(proj-core categories 2>/dev/null)}")
          (( $#cats )) && _describe 'category' cats
        fi
        ;;
      *)
        all=("${(@f)$(proj-core list --flat 2>/dev/null)}")
        (( $#all )) && _describe 'project' all
        ;;
    esac
  else
    local -a cmds
    cmds=(
      "sync:Scan for new/missing directories"
      "list:List projects grouped by category"
      "find:Search projects matching pattern"
      "go:Print path to a project directory"
      "rm:Move projects to removed"
      "mv:Move a project to a category"
      "mvt:Move multiple projects to a category"
      "clone:Clone a git repository"
      "init:Create a new project directory"
      "edit:Edit config with editor"
      "config:Show current config"
      "rename:Rename a project"
      "prune:Clean removed projects"
    )
    _describe -t commands 'command' cmds

    all=("${(@f)$(proj-core list --flat 2>/dev/null)}")
    (( $#all )) && _describe -t projects 'project' all
  fi
}
"#;



const PROJ_BASH_COMPLETION: &str = r#"_proj() {
  local cur="$2" prev="$3"
  local cmds="sync list find go mv mvt rename clone init edit rm prune config"

  if [[ $COMP_CWORD -eq 1 ]]; then
    COMPREPLY=($(compgen -W "$cmds" -- "$cur"))
    return
  fi

  case "${COMP_WORDS[1]}" in
    list|ls)
      COMPREPLY=($(compgen -W "$(proj-core categories 2>/dev/null)" -- "$cur"))
      ;;
    go|rm)
      COMPREPLY=($(compgen -W "$(proj-core list --names --all 2>/dev/null)" -- "$cur"))
      ;;
    mv)
      if [[ $COMP_CWORD -eq 2 ]]; then
        COMPREPLY=($(compgen -W "$(proj-core list --names --all 2>/dev/null)" -- "$cur"))
      else
        COMPREPLY=($(compgen -W "$(proj-core categories 2>/dev/null)" -- "$cur"))
      fi
      ;;
    mvt)
      if [[ $COMP_CWORD -eq 2 ]]; then
        COMPREPLY=($(compgen -W "$(proj-core categories 2>/dev/null)" -- "$cur"))
      else
        COMPREPLY=($(compgen -W "$(proj-core list --names --all 2>/dev/null)" -- "$cur"))
      fi
      ;;
    rename)
      if [[ $COMP_CWORD -eq 2 ]]; then
        COMPREPLY=($(compgen -W "$(proj-core list --names --all 2>/dev/null)" -- "$cur"))
      fi
      ;;
    clone|init)
      if [[ "$prev" == "--to" || "$prev" == "-t" ]]; then
        COMPREPLY=($(compgen -W "$(proj-core categories 2>/dev/null)" -- "$cur"))
      fi
      ;;
  esac
} && complete -F _proj proj
"#;

fn cmd_shell_func() {
    print!("{}", PROJ_SHELL.trim_start());
}

fn cmd_shell_completion(mode: ShellMode) {
    match mode {
        ShellMode::Zsh => {
            print!("{}", PROJ_ZSH_COMPLETION.trim_start());
            println!("\ncompdef _proj proj");
        }
        ShellMode::Zim => {
            print!("{}", PROJ_ZSH_COMPLETION.trim_start());
            println!("\n_proj");
        }
        ShellMode::Bash => print!("{}", PROJ_BASH_COMPLETION.trim_start()),
    }
}#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repo_name_from_url_ssh() {
        assert_eq!(repo_name_from_url("git@github.com:user/my-repo.git"), "my-repo");
    }

    #[test]
    fn test_repo_name_from_url_https() {
        assert_eq!(repo_name_from_url("https://github.com/user/my-repo.git"), "my-repo");
    }

    #[test]
    fn test_repo_name_from_url_no_git_suffix() {
        assert_eq!(repo_name_from_url("git@github.com:user/my-repo"), "my-repo");
    }

    #[test]
    fn test_repo_name_from_url_trailing_slash() {
        assert_eq!(repo_name_from_url("https://github.com/user/repo/"), "repo");
    }

    #[test]
    fn test_config_default_rm_to() {
        let cfg = Config::default();
        assert_eq!(cfg.rm_to, "removed");
    }

    #[test]
    fn test_config_default_init_to() {
        let cfg = Config::default();
        assert_eq!(cfg.init_to, "develop");
    }

    #[test]
    fn test_config_default_clone_to() {
        let cfg = Config::default();
        assert_eq!(cfg.clone_to, "uncategorized");
    }

    #[test]
    fn test_config_default_project_dir() {
        let cfg = Config::default();
        assert_eq!(cfg.project_dir, "~/Project");
    }

    #[test]
    fn test_config_default_use_fzf() {
        let cfg = Config::default();
        assert!(!cfg.use_fzf);
    }

    #[test]
    fn test_is_category_visible_all_visible_when_empty() {
        let cfg = Config {
            visible_categories: vec![],
            ..Config::default()
        };
        assert!(cfg.is_category_visible("anything"));
    }

    #[test]
    fn test_is_category_visible_matches() {
        let cfg = Config {
            visible_categories: vec!["stable".into(), "develop".into()],
            ..Config::default()
        };
        assert!(cfg.is_category_visible("stable"));
        assert!(cfg.is_category_visible("develop"));
        assert!(!cfg.is_category_visible("archived"));
    }

    #[test]
    fn test_format_sorted_order() {
        let mut projects = Projects::new();
        projects.insert("z-project".into(), "stable".into());
        projects.insert("a-project".into(), "develop".into());
        projects.insert("m-project".into(), "stable".into());

        let out = format_sorted(&projects);
        let lines: Vec<&str> = out.trim().split('\n').collect();

        // Should be sorted by category then name
        assert_eq!(lines[0], "a-project: develop");
        assert_eq!(lines[1], "m-project: stable");
        assert_eq!(lines[2], "z-project: stable");
    }

    #[test]
    fn test_known_categories_includes_config_values() {
        let cfg = read_settings();
        let cats = known_categories();
        assert!(cats.contains(&cfg.rm_to));
        assert!(cats.contains(&cfg.init_to));
        assert!(cats.contains(&cfg.clone_to));
    }
}
