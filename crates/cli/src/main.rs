use clap::{Parser, Subcommand};
use rust_ui_cli::{
    add, build, compare, load, materialize, mod_lines, resolve, to_json, FileState, COMPONENTS_DIR,
    THEME_FILE,
};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

/// Copy-paste components for the Rust UI kit.
#[derive(Parser)]
#[command(name = "rust-ui", version)]
struct Cli {
    /// Where to copy from: a local checkout of the Rust UI repo, or a git URL
    /// (cloned into the cache directory on first use).
    #[arg(long, global = true, default_value = ".", env = "RUST_UI_SOURCE")]
    source: String,
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// List every component in the registry.
    List,
    /// Regenerate registry.json from the component folders (`--check` only verifies it).
    Registry {
        #[arg(long)]
        check: bool,
    },
    /// Copy the theme stylesheet into a project.
    Init {
        /// Destination file for the theme CSS.
        #[arg(long, default_value = "assets/theme.css")]
        dest: PathBuf,
    },
    /// Copy components (and the components they depend on) into a project.
    Add {
        names: Vec<String>,
        /// Destination components directory.
        #[arg(long, default_value = "src/components")]
        dest: PathBuf,
        /// Overwrite files that differ.
        #[arg(long)]
        force: bool,
    },
    /// Scaffold a new component in this repo (files, mod/prelude lines, demo, tests, registry).
    New { name: String },
    /// Check components against the guardrails (colours, RTL, docs, demos, tests).
    Doctor {
        /// Also fail on warnings.
        #[arg(long)]
        strict: bool,
    },
    /// Print an accent preset (`[data-accent=...]`) generated from one brand colour.
    Theme {
        /// Brand colour as hex, e.g. `#7c3aed`.
        #[arg(long)]
        brand: String,
        /// Preset name used in `data-accent="..."`.
        #[arg(long, default_value = "brand")]
        name: String,
    },
    /// After `dx build --release --platform web`: drop debug names from the wasm, and put the
    /// theme and the component stylesheets the app uses into one file linked from index.html.
    Postbuild {
        /// The build's public folder, e.g. `target/dx/app/release/web/public`.
        public: PathBuf,
    },
    /// Show which copied files differ from the registry.
    Diff {
        names: Vec<String>,
        #[arg(long, default_value = "src/components")]
        dest: PathBuf,
    },
}

fn run(cli: Cli) -> Result<ExitCode, String> {
    let cache = std::env::var_os("XDG_CACHE_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".cache")))
        .unwrap_or_else(std::env::temp_dir)
        .join("rust-ui");
    let source = materialize(&cli.source, &cache)?;
    let root: &Path = &source;
    let err = |e: std::io::Error| e.to_string();
    match cli.command {
        Command::List => {
            for item in load(root)?.items {
                println!("{}", item.name);
            }
        }
        Command::Registry { check } => {
            let json = to_json(&build(root).map_err(err)?);
            let path = root.join("registry.json");
            if check {
                let stale = rust_ui_cli::manifest::stale_manifests(root).map_err(err)?;
                if std::fs::read_to_string(&path).ok().as_deref() != Some(json.as_str())
                    || !stale.is_empty()
                {
                    eprintln!(
                        "registry.json or a component.json is out of date; run `rust-ui registry`"
                    );
                    return Ok(ExitCode::FAILURE);
                }
            } else {
                std::fs::write(&path, json).map_err(err)?;
                println!("wrote {}", path.display());
                let n = rust_ui_cli::manifest::write_manifests(root).map_err(err)?;
                println!("wrote {n} component.json files");
                println!(
                    "wrote {}",
                    rust_ui_cli::docsgen::write(root).map_err(err)?.display()
                );
            }
        }
        Command::Init { dest } => {
            if let Some(parent) = dest.parent() {
                std::fs::create_dir_all(parent).map_err(err)?;
            }
            std::fs::copy(root.join(THEME_FILE), &dest).map_err(err)?;
            println!("wrote {}", dest.display());
        }
        Command::Add { names, dest, force } => {
            let items = resolve(&load(root)?, &names)?;
            let (written, skipped) = add(root, &items, &dest, force).map_err(err)?;
            println!("wrote {} file(s) to {}", written.len(), dest.display());
            for s in &skipped {
                println!("kept {} (differs; use --force to overwrite)", s.display());
            }
            println!("\nAdd to {}/mod.rs:\n{}", dest.display(), mod_lines(&items));
        }
        Command::New { name } => {
            let files = rust_ui_cli::scaffold::scaffold(root, &name)?;
            for f in &files {
                println!("wrote {}", f.display());
            }
            std::fs::write(
                root.join("registry.json"),
                to_json(&build(root).map_err(err)?),
            )
            .map_err(err)?;
            rust_ui_cli::manifest::write_manifests(root).map_err(err)?;
            rust_ui_cli::docsgen::write(root).map_err(err)?;
            println!("updated registry.json, component.json files and the docs index");
        }
        Command::Doctor { strict } => {
            use rust_ui_cli::doctor::{run as doctor, Level};
            let findings = doctor(root)?;
            let (mut errors, mut warnings) = (0, 0);
            for f in &findings {
                let tag = match f.level {
                    Level::Error => {
                        errors += 1;
                        "error"
                    }
                    Level::Warning => {
                        warnings += 1;
                        "warn "
                    }
                };
                println!("{tag} {}: {}", f.component, f.message);
            }
            println!("{errors} error(s), {warnings} warning(s)");
            if errors > 0 || (strict && warnings > 0) {
                return Ok(ExitCode::FAILURE);
            }
        }
        Command::Theme { brand, name } => {
            use rust_ui_cli::theme::{css, preset, Rgb};
            if !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') || name.is_empty() {
                return Err(format!("`{name}` must be letters, digits or `-`"));
            }
            print!("{}", css(&name, &preset(Rgb::parse(&brand)?)?));
        }
        Command::Postbuild { public } => {
            use rust_ui_cli::postbuild::{bundle_css, strip_wasm};
            let saved = strip_wasm(&public)?;
            println!("wasm: {} KB of names and debug info removed", saved / 1024);
            let css = bundle_css(&public)?;
            println!(
                "css: assets/{} holds {} stylesheets ({} unused left out)",
                css.file,
                css.held.len(),
                css.unused.len()
            );
        }
        Command::Diff { names, dest } => {
            let reg = load(root)?;
            let items = resolve(&reg, &names)?;
            let mut differs = false;
            for item in &items {
                for file in &item.files {
                    let state = compare(&root.join(COMPONENTS_DIR).join(file), &dest.join(file))
                        .map_err(err)?;
                    if state != FileState::Same {
                        differs = true;
                        println!("{state:?}\t{file}");
                    }
                }
            }
            if differs {
                return Ok(ExitCode::FAILURE);
            }
        }
    }
    Ok(ExitCode::SUCCESS)
}

fn main() -> ExitCode {
    match run(Cli::parse()) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}
