#![forbid(unsafe_code)]

use atoll_cli::update::{self, Lock};
use atoll_cli::{
    add_to, build, compare, load, materialize, mod_lines_of, pro_notice, resolve, to_json,
    FileState, Kind, Tier, THEME_FILE,
};
use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

/// Copy-paste components for the Atoll kit.
#[derive(Parser)]
#[command(name = "atoll", version)]
struct Cli {
    /// Where to copy from: a local checkout of the Atoll repo, or a git URL
    /// (cloned into the cache directory on first use).
    /// `update` defaults to the source recorded in the lockfile, everything else to `.`.
    #[arg(long, global = true, env = "ATOLL_SOURCE")]
    source: Option<String>,
    /// The lockfile `add` writes and `update` reads; the pristine copies `update` merges
    /// against go in `.atoll/base/` beside it.
    #[arg(long, global = true, default_value = atoll_cli::update::LOCK_FILE)]
    lock: PathBuf,
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// List every component and block in the registry (Pro items are marked).
    List,
    /// Regenerate registry.json from the component folders (`--check` only verifies it).
    Registry {
        #[arg(long)]
        check: bool,
    },
    /// Create a new Dioxus app that uses the kit: Cargo.toml, Dioxus.toml and a starter
    /// screen. Then `cd <dir> && dx serve`. Gets the kit from crates.io unless `--git` or
    /// `--path` is given (docs/GETTING_STARTED.md).
    Create {
        /// The folder to create (its last part is the package name unless `--name` is given).
        dir: PathBuf,
        /// The Cargo package name.
        #[arg(long)]
        name: Option<String>,
        /// Get the kit from git instead of crates.io (default URL: this repository).
        #[arg(long, num_args = 0..=1, default_missing_value = atoll_cli::create::REPO, conflicts_with = "path")]
        git: Option<String>,
        /// With `--git`: pin this commit.
        #[arg(long, requires = "git", conflicts_with = "tag")]
        rev: Option<String>,
        /// With `--git`: pin this tag, such as `v0.1.0`.
        #[arg(long, requires = "git")]
        tag: Option<String>,
        /// Use a local checkout of the kit's repository (for testing the kit itself).
        #[arg(long)]
        path: Option<PathBuf>,
        /// Start from one of the kit's example apps instead of the one-screen starter. Its
        /// files come from `--path`, the `--source` checkout, or a clone of the repository.
        #[arg(long, value_parser = template_names())]
        template: Option<String>,
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
    /// Run `cargo check` and explain the errors that come from using kit components in plain
    /// words: the missing or misspelt prop, its documentation, the enum variants that exist,
    /// and the component to `add`. Arguments after `--` go to `cargo check`.
    Explain {
        /// Read `cargo check --message-format=json` output from this file (`-` for stdin)
        /// instead of running cargo.
        #[arg(long)]
        from: Option<PathBuf>,
        /// Your copied components, read instead of the registry's versions.
        #[arg(long, default_value = "src/components")]
        dest: PathBuf,
        #[arg(long, default_value = "src/blocks")]
        blocks_dest: PathBuf,
        /// Print the hints as JSON (for editors and agents).
        #[arg(long)]
        json: bool,
        /// Passed to `cargo check`, e.g. `-- -p my-app --features web`.
        #[arg(last = true)]
        cargo_args: Vec<String>,
    },
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
    /// Audit what every animation and transition costs the browser (composite, paint or
    /// layout). Layout animations fail unless an `atoll-motion: layout` comment explains them.
    Motion {
        /// Stylesheets to audit instead of the kit's own (repeatable).
        #[arg(long)]
        css: Vec<PathBuf>,
        /// List every animated property, not only the layout ones.
        #[arg(long)]
        all: bool,
    },
    /// Print an accent preset (`[data-accent=...]`) generated from one brand colour, or from
    /// the colours of a logo.
    Theme {
        /// Brand colour as hex, e.g. `#7c3aed`.
        #[arg(
            long,
            required_unless_present = "from_image",
            conflicts_with = "from_image"
        )]
        brand: Option<String>,
        /// A logo (PNG, JPEG or SVG): its most visible saturated colour becomes the brand
        /// colour, and a clearly different second colour is printed as an alternative preset.
        #[arg(long)]
        from_image: Option<PathBuf>,
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
    /// Describe the app for a coding agent: Dioxus version, theme, installed items and which
    /// of them you edited, skill and MCP setup. Reads only the app; `--json` for agents.
    Info {
        /// The app to inspect.
        #[arg(long, default_value = ".")]
        project: PathBuf,
        #[arg(long)]
        json: bool,
    },
    /// Install the agent skill (`skills/atoll/SKILL.md`) into the app, or print it.
    Skill {
        /// Where to write it; Claude Code reads `.claude/skills/`, other agents may use
        /// `.agents/skills/atoll/SKILL.md`.
        #[arg(long, default_value = atoll_cli::info::SKILL_DEST)]
        dest: PathBuf,
        /// Print it instead of writing it.
        #[arg(long)]
        print: bool,
    },
    /// Print the keyboard and accessibility contracts (`contract.toml`): the docs table, or
    /// with `--json` every row's test steps, which the keyboard test runners read.
    Contract {
        /// Only this component.
        name: Option<String>,
        #[arg(long)]
        json: bool,
    },
    /// Write a draft Accessibility Conformance Report (VPAT 2.5 layout, WCAG 2.2 A and AA) for
    /// the kit components your app uses, from the kit's tests and keyboard contracts.
    /// docs/ACR.md explains what it covers.
    Acr {
        /// Components or blocks to report on (with what they use). Default: the items in the
        /// lockfile, or every component when there is none.
        names: Vec<String>,
        /// The product name in the title.
        #[arg(long, default_value = "Your app")]
        product: String,
        /// Write the Markdown here instead of printing it.
        #[arg(long)]
        out: Option<PathBuf>,
        /// The report date (YYYY-MM-DD); today by default.
        #[arg(long)]
        date: Option<String>,
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
        .join("atoll");
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
                let mut tags = Vec::new();
                if item.kind == Kind::Block {
                    tags.push("block");
                }
                if item.tier == Tier::Pro {
                    tags.push("pro");
                }
                if tags.is_empty() {
                    println!("{}", item.name);
                } else {
                    println!("{} ({})", item.name, tags.join(", "));
                }
            }
        }
        Command::Registry { check } => {
            let json = to_json(&build(root).map_err(err)?);
            let path = root.join("registry.json");
            if check {
                let stale = atoll_cli::manifest::stale_manifests(root).map_err(err)?;
                let llms = atoll_cli::llms::stale(root).map_err(err)?;
                let scoped = atoll_cli::scope::stale(root).map_err(err)?;
                if std::fs::read_to_string(&path).ok().as_deref() != Some(json.as_str())
                    || !stale.is_empty()
                    || !llms.is_empty()
                    || !scoped.is_empty()
                {
                    eprintln!(
                        "registry.json, a component.json, the llms.txt files or a *.scoped.css theme are out of date; run `atoll registry`"
                    );
                    return Ok(ExitCode::FAILURE);
                }
            } else {
                std::fs::write(&path, json).map_err(err)?;
                println!("wrote {}", path.display());
                let n = atoll_cli::manifest::write_manifests(root).map_err(err)?;
                println!("wrote {n} component.json files");
                println!(
                    "wrote {}",
                    atoll_cli::docsgen::write(root).map_err(err)?.display()
                );
                let n = atoll_cli::llms::write(root).map_err(err)?;
                println!("wrote {n} llms.txt files");
                for p in atoll_cli::scope::write(root).map_err(err)? {
                    println!("wrote {}", p.display());
                }
            }
        }
        Command::Create {
            dir,
            name,
            git,
            rev,
            tag,
            path,
            template,
        } => {
            let name = match name {
                Some(n) => n,
                None => dir
                    .file_name()
                    .and_then(|n| n.to_str())
                    .map(str::to_string)
                    .ok_or_else(|| {
                        format!(
                            "can't take a package name from {}; pass --name",
                            dir.display()
                        )
                    })?,
            };
            let source = match (git, path) {
                (Some(url), _) => atoll_cli::create::KitSource::Git { url, rev, tag },
                (None, Some(p)) => atoll_cli::create::KitSource::Path(
                    std::fs::canonicalize(&p).map_err(|e| format!("{}: {e}", p.display()))?,
                ),
                (None, None) => atoll_cli::create::KitSource::Registry,
            };
            let written = match template {
                Some(t) => {
                    let t = atoll_cli::template::find(&t)?;
                    let kit = template_source(&source, root, t, &cache)?;
                    atoll_cli::template::create(&dir, &name, &source, t, &kit)?
                }
                None => atoll_cli::create::create(&dir, &name, &source)?,
            };
            for f in written {
                println!("wrote {}", f.display());
            }
            println!("\nNext:\n  cd {}\n  dx serve        # http://localhost:8080\n\nAdd components with `atoll add <name>` to own their source, or use them from `ui::prelude` as they are.", dir.display());
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
            if let Some(notice) = pro_notice(&items) {
                println!("\n{notice}");
            }
        }
        Command::New { name } => {
            let files = atoll_cli::scaffold::scaffold(root, &name)?;
            for f in &files {
                println!("wrote {}", f.display());
            }
            std::fs::write(
                root.join("registry.json"),
                to_json(&build(root).map_err(err)?),
            )
            .map_err(err)?;
            atoll_cli::manifest::write_manifests(root).map_err(err)?;
            atoll_cli::docsgen::write(root).map_err(err)?;
            atoll_cli::llms::write(root).map_err(err)?;
            println!("updated registry.json, component.json files, the docs index and llms.txt");
        }
        Command::Explain {
            from,
            dest,
            blocks_dest,
            json,
            cargo_args,
        } => {
            use atoll_cli::explain::{render, Api};
            let output = match from {
                Some(p) if p.as_os_str() == "-" => {
                    std::io::read_to_string(std::io::stdin()).map_err(err)?
                }
                Some(p) => {
                    std::fs::read_to_string(&p).map_err(|e| format!("{}: {e}", p.display()))?
                }
                None => {
                    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".into());
                    let out = std::process::Command::new(cargo)
                        .args(["check", "--message-format=json"])
                        .args(&cargo_args)
                        .stderr(std::process::Stdio::inherit())
                        .output()
                        .map_err(|e| format!("could not run cargo: {e}"))?;
                    String::from_utf8_lossy(&out.stdout).into_owned()
                }
            };
            let api = Api::load(root, &[dest, blocks_dest]);
            let cwd = std::env::current_dir().map_err(err)?;
            let report = api.explain(&output, &cwd);
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&report).map_err(|e| e.to_string())?
                );
            } else if report.hints.is_empty() && report.other.is_empty() {
                println!("no errors");
            } else {
                print!("{}", render(&report));
            }
            if !report.hints.is_empty() || !report.other.is_empty() {
                return Ok(ExitCode::FAILURE);
            }
        }
        Command::Changelog { check } => {
            use atoll_cli::changelog::{fold, load, unnamed};
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
            use atoll_cli::doctor::{run as doctor, Level};
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
            use atoll_cli::contrast::{run as contrast, summary};
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
        Command::Motion { css, all } => {
            use atoll_cli::motion::{run as motion, Cost};
            let report = motion(root, &css)?;
            for f in &report.findings {
                if all || f.cost == Cost::Layout {
                    let tag = match (f.cost, &f.allowed) {
                        (Cost::Layout, None) => "fail",
                        (Cost::Layout, Some(_)) => "allowed",
                        _ => "ok",
                    };
                    println!("{tag} {f}");
                }
            }
            let errors = report.errors().len();
            println!(
                "{} stylesheets: {} composite, {} paint, {} layout ({} explained, {errors} not)",
                report.files,
                report.count(Cost::Composite),
                report.count(Cost::Paint),
                report.count(Cost::Layout),
                report.count(Cost::Layout) - errors,
            );
            if errors > 0 {
                return Ok(ExitCode::FAILURE);
            }
        }
        Command::Theme {
            brand,
            from_image,
            name,
        } => {
            use atoll_cli::theme::{css, preset, Rgb};
            if !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') || name.is_empty() {
                return Err(format!("`{name}` must be letters, digits or `-`"));
            }
            match (brand, from_image) {
                (Some(brand), _) => print!("{}", css(&name, &preset(Rgb::parse(&brand)?)?)),
                (None, Some(logo)) => {
                    let palette = atoll_cli::logo::read(&logo)?;
                    print!("{}", atoll_cli::logo::summary(&palette));
                    print!("{}", css(&name, &preset(palette.brand)?));
                    if let Some(second) = palette.second {
                        println!(
                            "\n/* Alternative: the logo's second colour ({}). */",
                            second.hex()
                        );
                        print!("{}", css(&format!("{name}-2"), &preset(second)?));
                    }
                }
                (None, None) => return Err("give --brand or --from-image".into()),
            }
        }
        Command::Postbuild { public, scoped } => {
            use atoll_cli::postbuild::{bundle_css_with, strip_wasm};
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
        Command::Info { project, json } => {
            let info = atoll_cli::info::gather(&project, &cli.lock)?;
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&info).map_err(|e| e.to_string())?
                );
            } else {
                print!("{}", atoll_cli::info::render(&info));
            }
        }
        Command::Skill { dest, print } => {
            let skill =
                std::fs::read_to_string(root.join(atoll_cli::info::SKILL_FILE)).map_err(|e| {
                    format!(
                        "{} in {}: {e}; pass --source <the kit's git URL or checkout>",
                        atoll_cli::info::SKILL_FILE,
                        root.display()
                    )
                })?;
            if print {
                print!("{skill}");
            } else {
                if let Some(parent) = dest.parent() {
                    std::fs::create_dir_all(parent).map_err(err)?;
                }
                std::fs::write(&dest, skill).map_err(err)?;
                println!("wrote {}", dest.display());
            }
        }
        Command::Docs { name } => {
            use atoll_cli::llms::{index, page};
            let reg = build(root).map_err(err)?;
            match name {
                None => print!("{}", index(root, &reg).map_err(err)?),
                Some(name) => match page(root, &reg, &name).map_err(err)? {
                    Some(text) => print!("{text}"),
                    None => {
                        return Err(format!(
                            "unknown component or block `{name}`; `atoll list` shows them all"
                        ))
                    }
                },
            }
        }
        Command::Contract { name, json } => {
            use atoll_cli::contract::Contract;
            let all = Contract::load_all(&root.join(atoll_cli::COMPONENTS_DIR))?;
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
        Command::Acr {
            names,
            product,
            out,
            date,
        } => {
            use atoll_cli::acr::{components_of, load_criteria, markdown, report};
            let reg = load(root)?;
            let wanted: Vec<String> = if !names.is_empty() {
                names
            } else {
                let lock = Lock::load(&cli.lock)?;
                if lock.items.is_empty() {
                    reg.items
                        .iter()
                        .filter(|i| i.kind == Kind::Component)
                        .map(|i| i.name.clone())
                        .collect()
                } else {
                    lock.items.keys().cloned().collect()
                }
            };
            let components = components_of(&reg, &wanted)?;
            let criteria = load_criteria(root)?;
            let date = date.unwrap_or_else(atoll_cli::acr::today);
            let text = markdown(&report(root, &criteria, &components, &product)?, &date);
            match out {
                Some(path) => {
                    std::fs::write(&path, text).map_err(err)?;
                    println!(
                        "wrote {} ({} components); read it and fill in the rows marked review",
                        path.display(),
                        components.len()
                    );
                }
                None => print!("{text}"),
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
                    "{} lists nothing to update; install with `atoll add` first",
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
            atoll_cli::mcp::Server::new(root, project)
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

/// `--template` choices, with each template's one-line summary for `--help`.
fn template_names() -> clap::builder::PossibleValuesParser {
    clap::builder::PossibleValuesParser::new(
        atoll_cli::template::TEMPLATES
            .iter()
            .map(|t| clap::builder::PossibleValue::new(t.name).help(t.summary)),
    )
}

/// The kit checkout a template's files are copied from: `--path`, else the `--source`
/// checkout when it has the example, else a clone of `--git` or of the kit's repository.
fn template_source(
    kit: &atoll_cli::create::KitSource,
    root: &Path,
    template: &atoll_cli::template::Template,
    cache: &Path,
) -> Result<PathBuf, String> {
    use atoll_cli::create::{KitSource, REPO};
    let has_it = |p: &Path| {
        p.join("apps")
            .join(template.app)
            .join("Cargo.toml")
            .exists()
    };
    if let KitSource::Path(p) = kit {
        return Ok(p.clone());
    }
    if has_it(root) {
        return Ok(root.to_path_buf());
    }
    let url = match kit {
        KitSource::Git { url, .. } => url.clone(),
        _ => REPO.to_string(),
    };
    let dir = materialize(&url, cache)?;
    if has_it(&dir) {
        Ok(dir)
    } else {
        Err(format!(
            "{url} has no apps/{}; pass --path <kit checkout>",
            template.app
        ))
    }
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

/// `atoll init`: copy the theme (scoped or not, plus the reset if asked), then check the
/// app it lands in.
fn init(root: &Path, app: &Path, dest: &Path, scoped: bool, reset: bool) -> Result<(), String> {
    use atoll_cli::{app as wiring, scope};
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

/// After `atoll add`: declare the copied modules and add the crates they need, so the app
/// builds without hand edits.
fn wire(
    root: &Path,
    app: &Path,
    items: &[atoll_cli::Item],
    dest: &Path,
    blocks_dest: &Path,
) -> Result<(), String> {
    use atoll_cli::app::{self as wiring, DepChange};
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
