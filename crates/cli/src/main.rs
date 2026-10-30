use clap::{Parser, Subcommand};
use rust_ui_cli::update::{self, Lock};
use rust_ui_cli::{
    add_to, build, compare, load, materialize, mod_lines_of, resolve, to_json, FileState, Kind,
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
    /// `update` defaults to the source recorded in the lockfile, everything else to `.`.
    #[arg(long, global = true, env = "RUST_UI_SOURCE")]
    source: Option<String>,
    /// The lockfile `add` writes and `update` reads; the pristine copies `update` merges
    /// against go in `.rust-ui/base/` beside it.
    #[arg(long, global = true, default_value = rust_ui_cli::update::LOCK_FILE)]
    lock: PathBuf,
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// List every component and block in the registry.
    List,
    /// Regenerate registry.json from the component folders (`--check` only verifies it).
    Registry {
        #[arg(long)]
        check: bool,
    },
    /// Copy the theme stylesheet into a project, new or existing: checks the Dioxus version,
    /// reports CSS in the app that clashes with the kit's, and prints how to load the theme.
    Init {
        /// Destination file for the theme CSS.
        #[arg(long, default_value = "assets/theme.css")]
        dest: PathBuf,
        /// Confine the theme to `ThemeScope` (data-rui-scope), for an app with its own CSS.
        #[arg(long)]
        scoped: bool,
        /// Also copy the small base stylesheet (`reset.css`) beside the theme.
        #[arg(long)]
        reset: bool,
        /// The app's root folder (where its Cargo.toml is).
        #[arg(long, default_value = ".")]
        app: PathBuf,
    },
    /// Copy components or blocks (and the components they depend on) into a project.
    Add {
        names: Vec<String>,
        /// Destination components directory.
        #[arg(long, default_value = "src/components")]
        dest: PathBuf,
        /// Destination for blocks (whole screens such as `login_01`).
        #[arg(long, default_value = "src/blocks")]
        blocks_dest: PathBuf,
        /// Overwrite files that differ.
        #[arg(long)]
        force: bool,
        /// Only copy files: don't edit `mod.rs`, `main.rs`/`lib.rs` or `Cargo.toml`.
        #[arg(long)]
        no_wire: bool,
        /// The app's root folder (where its Cargo.toml is).
        #[arg(long, default_value = ".")]
        app: PathBuf,
    },
    /// Scaffold a new component in this repo (files, demo, test, gallery section, registry).
    New { name: String },
    /// Fold the per-PR fragments in `changes/` into CHANGELOG.md and docs/MIGRATION.md
    /// (`--check` only validates them). Run on `main` before a release; changes/README.md.
    Changelog {
        #[arg(long)]
        check: bool,
    },
    /// Check components against the guardrails (colours, RTL, docs, demos, tests).
    Doctor {
        /// Also fail on warnings.
        #[arg(long)]
        strict: bool,
    },
    /// Prove every text colour the components draw is readable (WCAG AA) on the background
    /// it is drawn on, in light and dark, under every accent preset.
    Contrast {
        /// Your own theme or preset files, loaded after the kit's theme (repeatable).
        #[arg(long)]
        theme: Vec<PathBuf>,
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
        /// The app renders `ThemeStyles { scoped: true }`: bundle the scoped theme instead.
        #[arg(long)]
        scoped: bool,
    },
    /// Print a component's page for coding agents (docs, props, example), or the index
    /// (`llms.txt`) without a name.
    Docs { name: Option<String> },
    /// Print the keyboard and accessibility contracts (`contract.toml`): the docs table, or
    /// with `--json` every row's test steps, which the keyboard test runners read.
    Contract {
        /// Only this component.
        name: Option<String>,
        #[arg(long)]
        json: bool,
    },
    /// Update copied components and blocks to the registry's current version, keeping your
    /// edits: a three-way merge against the version `add` recorded in the lockfile.
    Update {
        /// Items to update (and any components they now need); all installed ones when empty.
        names: Vec<String>,
        /// Show what would change, as a diff, without writing anything.
        #[arg(long)]
        dry_run: bool,
        /// Where newly needed components go when none is installed yet.
        #[arg(long, default_value = "src/components")]
        dest: PathBuf,
        #[arg(long, default_value = "src/blocks")]
        blocks_dest: PathBuf,
    },
    /// Serve the registry to AI coding tools over the Model Context Protocol (stdio):
    /// list, search, read and add components. docs/MCP.md shows the client setup.
    Mcp {
        /// The app that `add_component` and `init_theme` write into.
        #[arg(long, default_value = ".")]
        project: PathBuf,
    },
    /// Show which copied files differ from the registry.
    Diff {
        names: Vec<String>,
        #[arg(long, default_value = "src/components")]
        dest: PathBuf,
        #[arg(long, default_value = "src/blocks")]
        blocks_dest: PathBuf,
    },
}

fn run(cli: Cli) -> Result<ExitCode, String> {
    let cache = std::env::var_os("XDG_CACHE_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".cache")))
        .unwrap_or_else(std::env::temp_dir)
        .join("rust-ui");
    let source_arg = match (&cli.source, &cli.command) {
        (Some(s), _) => s.clone(),
        (None, Command::Update { .. }) => {
            Lock::load(&cli.lock)?.source.unwrap_or_else(|| ".".into())
        }
        (None, _) => ".".into(),
    };
    let source = materialize(&source_arg, &cache)?;
    let root: &Path = &source;
    let err = |e: std::io::Error| e.to_string();
    match cli.command {
        Command::List => {
            for item in load(root)?.items {
                match item.kind {
                    Kind::Component => println!("{}", item.name),
                    Kind::Block => println!("{} (block)", item.name),
                }
            }
        }
        Command::Registry { check } => {
            let json = to_json(&build(root).map_err(err)?);
            let path = root.join("registry.json");
            if check {
                let stale = rust_ui_cli::manifest::stale_manifests(root).map_err(err)?;
                let llms = rust_ui_cli::llms::stale(root).map_err(err)?;
                let scoped = rust_ui_cli::scope::stale(root).map_err(err)?;
                if std::fs::read_to_string(&path).ok().as_deref() != Some(json.as_str())
                    || !stale.is_empty()
                    || !llms.is_empty()
                    || !scoped.is_empty()
                {
                    eprintln!(
                        "registry.json, a component.json, the llms.txt files or a *.scoped.css theme are out of date; run `rust-ui registry`"
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
                let n = rust_ui_cli::llms::write(root).map_err(err)?;
                println!("wrote {n} llms.txt files");
                for p in rust_ui_cli::scope::write(root).map_err(err)? {
                    println!("wrote {}", p.display());
                }
            }
        }
        Command::Init {
            dest,
            scoped,
            reset,
            app,
        } => init(root, &app, &dest, scoped, reset)?,
        Command::Add {
            names,
            dest,
            blocks_dest,
            force,
            no_wire,
            app,
        } => {
            let items = resolve(&load(root)?, &names)?;
            let (written, skipped) =
                add_to(root, &items, &dest, &blocks_dest, force).map_err(err)?;
            println!("wrote {} file(s)", written.len());
            let recorded =
                update::record(&cli.lock, root, &source_arg, &items, &dest, &blocks_dest)?;
            println!("recorded {recorded} file(s) in {}", cli.lock.display());
            for s in &skipped {
                println!("kept {} (differs; use --force to overwrite)", s.display());
            }
            if no_wire {
                for (kind, dir) in [(Kind::Component, &dest), (Kind::Block, &blocks_dest)] {
                    let lines = mod_lines_of(&items, kind);
                    if !lines.is_empty() {
                        println!("\nAdd to {}/mod.rs:\n{lines}", dir.display());
                    }
                }
            } else {
                wire(root, &app, &items, &dest, &blocks_dest)?;
            }
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
            rust_ui_cli::llms::write(root).map_err(err)?;
            println!("updated registry.json, component.json files, the docs index and llms.txt");
        }
        Command::Changelog { check } => {
            use rust_ui_cli::changelog::{fold, load, unnamed};
            if check {
                let fragments = load(root)?;
                for p in unnamed(&fragments) {
                    println!(
                        "warn {}: no PR number yet; add `(#N)` once the PR is open",
                        p.display()
                    );
                }
                println!("{} fragment(s) OK", fragments.len());
            } else {
                let folded = fold(root)?;
                println!(
                    "folded {} fragment(s) into CHANGELOG.md and docs/MIGRATION.md",
                    folded.len()
                );
            }
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
        Command::Contrast { theme } => {
            use rust_ui_cli::contrast::{run as contrast, summary};
            let report = contrast(root, &theme)?;
            let failures = summary(&report);
            for f in &failures {
                println!("fail {f}");
            }
            println!(
                "{} pairs checked, {} rules failing",
                report.pairs.len(),
                failures.len()
            );
            if !failures.is_empty() {
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
        Command::Postbuild { public, scoped } => {
            use rust_ui_cli::postbuild::{bundle_css_with, strip_wasm};
            let saved = strip_wasm(&public)?;
            println!("wasm: {} KB of names and debug info removed", saved / 1024);
            let css = bundle_css_with(&public, scoped)?;
            println!(
                "css: assets/{} holds {} stylesheets ({} unused left out)",
                css.file,
                css.held.len(),
                css.unused.len()
            );
        }
        Command::Docs { name } => {
            use rust_ui_cli::llms::{index, page};
            let reg = build(root).map_err(err)?;
            match name {
                None => print!("{}", index(root, &reg).map_err(err)?),
                Some(name) => match page(root, &reg, &name).map_err(err)? {
                    Some(text) => print!("{text}"),
                    None => {
                        return Err(format!(
                            "unknown component or block `{name}`; `rust-ui list` shows them all"
                        ))
                    }
                },
            }
        }
        Command::Contract { name, json } => {
            use rust_ui_cli::contract::Contract;
            let all = Contract::load_all(&root.join(rust_ui_cli::COMPONENTS_DIR))?;
            let picked: Vec<&Contract> = all
                .iter()
                .filter(|c| name.as_deref().is_none_or(|n| n == c.name))
                .collect();
            if let (Some(n), true) = (&name, picked.is_empty()) {
                return Err(format!("`{n}` has no contract.toml"));
            }
            if json {
                let runnable = picked
                    .iter()
                    .map(|c| c.runnable())
                    .collect::<Result<Vec<_>, _>>()?;
                println!(
                    "{}",
                    serde_json::to_string_pretty(&runnable).map_err(|e| e.to_string())?
                );
            } else {
                for c in picked {
                    println!("# {}\n\n{}", c.name, c.markdown());
                }
            }
        }
        Command::Update {
            names,
            dry_run,
            dest,
            blocks_dest,
        } => {
            use update::Action;
            let lock = Lock::load(&cli.lock)?;
            if lock.items.is_empty() {
                return Err(format!(
                    "{} lists nothing to update; install with `rust-ui add` first",
                    cli.lock.display()
                ));
            }
            let reg = load(root)?;
            let plan = update::plan(root, &reg, &lock, &cli.lock, &names, &dest, &blocks_dest)?;
            for f in plan.files.iter().filter(|f| f.action != Action::UpToDate) {
                let note = f.action.note();
                let sep = if note.is_empty() { "" } else { " - " };
                println!("{:<10} {}{sep}{note}", f.action.label(), f.path.display());
            }
            for name in &plan.gone {
                println!("gone       {name} - no longer in the registry; left as it is");
            }
            let counts = [
                Action::Updated,
                Action::Merged,
                Action::Added,
                Action::Conflict,
                Action::Unresolved,
                Action::NoBase,
                Action::KeptEdits,
                Action::UpToDate,
            ]
            .into_iter()
            .filter(|a| plan.count(*a) > 0)
            .map(|a| format!("{} {}", plan.count(a), a.label().to_lowercase()))
            .collect::<Vec<_>>()
            .join(", ");
            if dry_run {
                print!("{}", update::render_diff(&plan));
                println!("dry run: {counts}; nothing written");
            } else {
                update::apply(&plan, &cli.lock, &source_arg)?;
                println!("{counts}");
                for (item, dir) in &plan.new_items {
                    println!("Add to {dir}/mod.rs: pub mod {};", item.name);
                }
            }
            if plan.has_conflicts() {
                return Ok(ExitCode::FAILURE);
            }
        }
        Command::Mcp { project } => {
            let stdin = std::io::stdin();
            rust_ui_cli::mcp::Server::new(root, project)
                .with_source(source_arg.as_str())
                .serve(stdin.lock(), std::io::stdout().lock())
                .map_err(err)?;
        }
        Command::Diff {
            names,
            dest,
            blocks_dest,
        } => {
            let reg = load(root)?;
            let items = resolve(&reg, &names)?;
            let mut differs = false;
            for item in &items {
                let target = match item.kind {
                    Kind::Component => &dest,
                    Kind::Block => &blocks_dest,
                };
                for file in &item.files {
                    let state = compare(&root.join(item.kind.dir()).join(file), &target.join(file))
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

/// `rust-ui init`: copy the theme (scoped or not, plus the reset if asked), then check the
/// app it lands in.
fn init(root: &Path, app: &Path, dest: &Path, scoped: bool, reset: bool) -> Result<(), String> {
    use rust_ui_cli::{app as wiring, scope};
    let err = |e: std::io::Error| e.to_string();
    let write = |to: &Path, from: &str| -> Result<(), String> {
        let css = std::fs::read_to_string(root.join(from)).map_err(err)?;
        let name = Path::new(from)
            .file_name()
            .unwrap_or_default()
            .to_string_lossy();
        let text = if scoped {
            scope::scoped_file(&name, &css)
        } else {
            css
        };
        if let Some(parent) = to.parent() {
            std::fs::create_dir_all(parent).map_err(err)?;
        }
        std::fs::write(to, text).map_err(err)?;
        println!(
            "wrote {}{}",
            to.display(),
            if scoped { " (scoped)" } else { "" }
        );
        Ok(())
    };
    write(dest, THEME_FILE)?;
    let reset_dest = dest.with_file_name("reset.css");
    if reset {
        write(&reset_dest, "crates/ui/assets/reset.css")?;
    }

    let manifest = app.join("Cargo.toml");
    match wiring::dioxus_version(&manifest) {
        Some(v) if wiring::on_supported_line(&v) => println!("dioxus {v}: supported"),
        Some(v) => println!(
            "warn: this app is on dioxus {v}; the kit is built and tested on {}.x only",
            wiring::DIOXUS_LINE
        ),
        None if manifest.is_file() => {
            println!("warn: no dioxus dependency in {}", manifest.display())
        }
        None => {}
    }

    let theme = std::fs::read_to_string(root.join(THEME_FILE)).map_err(err)?;
    let clashes = wiring::clashes(app, &theme, &[dest.to_path_buf(), reset_dest.clone()]);
    if !clashes.is_empty() {
        println!(
            "\nYour CSS defines {} thing(s) the kit's theme also defines:",
            clashes.len()
        );
        for c in clashes.iter().take(20) {
            println!("  {}:{}  {}", c.file.display(), c.line, c.what);
        }
        if clashes.len() > 20 {
            println!("  ... and {} more", clashes.len() - 20);
        }
        if scoped {
            println!("The scoped theme keeps them apart: the kit's apply only inside ThemeScope.");
        } else {
            println!(
                "Re-run with --scoped to keep the kit's tokens inside ThemeScope (docs/EXISTING_APP.md)."
            );
        }
    }
    if wiring::uses_tailwind(app) {
        println!(
            "\nTailwind found: it and the kit don't share class names (the kit's are hashed CSS modules).{}",
            if reset {
                " Its preflight already resets the page; you can drop reset.css."
            } else {
                ""
            }
        );
    }

    let asset = |p: &Path| {
        let rel = p
            .strip_prefix(app)
            .unwrap_or(p)
            .to_string_lossy()
            .replace('\\', "/");
        format!("asset!(\"/{}\")", rel.trim_start_matches("./"))
    };
    println!("\nLoad it once near the root of the app:\n");
    println!("    document::Stylesheet {{ href: {} }}", asset(dest));
    if reset {
        println!(
            "    document::Stylesheet {{ href: {} }}",
            asset(&reset_dest)
        );
    }
    if scoped {
        println!("    // copied theme_customizer: call set_writes_html(false) once at startup");
        println!("    ThemeScope {{ /* the part of the app that uses the kit */ }}");
        println!(
            "\n(With the kit as a crate instead: ThemeStyles {{ scoped: true }} + ThemeScope.)"
        );
    }
    Ok(())
}

/// After `rust-ui add`: declare the copied modules and add the crates they need, so the app
/// builds without hand edits.
fn wire(
    root: &Path,
    app: &Path,
    items: &[rust_ui_cli::Item],
    dest: &Path,
    blocks_dest: &Path,
) -> Result<(), String> {
    use rust_ui_cli::app::{self as wiring, DepChange};
    let err = |e: std::io::Error| e.to_string();
    for (kind, dir) in [(Kind::Component, dest), (Kind::Block, blocks_dest)] {
        let names: Vec<String> = items
            .iter()
            .filter(|i| i.kind == kind)
            .map(|i| i.name.clone())
            .collect();
        if names.is_empty() {
            continue;
        }
        let mod_rs = dir.join("mod.rs");
        let added = wiring::ensure_mod_lines(&mod_rs, &names).map_err(err)?;
        if !added.is_empty() {
            println!("declared {} in {}", added.join(", "), mod_rs.display());
        }
        // `src/components` -> `mod components;` in the crate root.
        let module = dir.file_name().map(|n| n.to_string_lossy().into_owned());
        let in_src = dir.parent().is_some_and(|p| same_dir(p, &app.join("src")));
        if let (Some(module), true) = (module, in_src) {
            if let Some(f) = wiring::ensure_root_mod(app, &module).map_err(err)? {
                println!("added `mod {module};` to {}", f.display());
            }
        }
    }
    let manifest = app.join("Cargo.toml");
    if manifest.is_file() {
        let deps = wiring::cargo_deps(root, items)?;
        for change in wiring::add_cargo_deps(&manifest, &deps)? {
            match change {
                DepChange::Added(name) => println!("added {name} to {}", manifest.display()),
                DepChange::Features(name, f) => {
                    println!("added features {} to {name}", f.join(", "))
                }
                DepChange::Differs { name, ours } => {
                    println!("kept your {name}; the registry's components are tested with {ours}")
                }
            }
        }
    }
    Ok(())
}

fn same_dir(a: &Path, b: &Path) -> bool {
    match (a.canonicalize(), b.canonicalize()) {
        (Ok(a), Ok(b)) => a == b,
        _ => a == b,
    }
}
