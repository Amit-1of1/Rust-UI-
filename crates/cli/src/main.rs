use clap::{Parser, Subcommand};
use rust_ui_cli::{
    add, build, compare, load, mod_lines, resolve, to_json, FileState, COMPONENTS_DIR, THEME_FILE,
};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

/// Copy-paste components for the Rust UI kit.
#[derive(Parser)]
#[command(name = "rust-ui", version)]
struct Cli {
    /// Checkout of the Rust UI repo to copy from.
    #[arg(long, global = true, default_value = ".", env = "RUST_UI_SOURCE")]
    source: PathBuf,
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
    /// Show which copied files differ from the registry.
    Diff {
        names: Vec<String>,
        #[arg(long, default_value = "src/components")]
        dest: PathBuf,
    },
}

fn run(cli: Cli) -> Result<ExitCode, String> {
    let root: &Path = &cli.source;
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
                if std::fs::read_to_string(&path).ok().as_deref() != Some(json.as_str()) {
                    eprintln!("registry.json is out of date; run `rust-ui registry`");
                    return Ok(ExitCode::FAILURE);
                }
            } else {
                std::fs::write(&path, json).map_err(err)?;
                println!("wrote {}", path.display());
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
