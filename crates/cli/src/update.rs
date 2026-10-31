//! `rust-ui update`: bring copied components up to the registry's current version without
//! losing local edits.
//!
//! `rust-ui add` records what it copied in `rust-ui-lock.json` (which registry commit each
//! item came from and a hash of each file) and keeps a pristine copy of each file under
//! `.rust-ui/base/`. `update` then does a three-way merge per file: the recorded copy is the
//! common ancestor, the project's file is "yours" and the registry's current file is the other
//! side. Untouched files are replaced, edits that don't overlap the registry's changes are
//! merged, and overlapping ones get conflict markers, as `git merge` does.

use crate::{resolve, Item, Kind, Registry};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Default lockfile name, in the project root (the directory `rust-ui` runs in).
pub const LOCK_FILE: &str = "rust-ui-lock.json";
/// Pristine copies of the installed files, next to the lockfile.
pub const BASE_DIR: &str = ".rust-ui/base";
/// The first line of a conflict this command writes.
pub const CONFLICT_START: &str = "<<<<<<< yours";

/// What `rust-ui add` and `rust-ui update` installed, keyed by item name.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Lock {
    pub version: u32,
    /// The `--source` the items were last installed from; `update` uses it when no
    /// `--source` is given.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    pub items: BTreeMap<String, LockItem>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LockItem {
    #[serde(default, skip_serializing_if = "Kind::is_component")]
    pub kind: Kind,
    /// The folder the item was copied into (`src/components`), relative to the project root
    /// (the lockfile's folder).
    pub dir: String,
    /// The source's git commit when the item was installed or last updated, if it had one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rev: Option<String>,
    /// Hash of each file as installed (`<item>/<file>` -> FNV-1a 64 hex). Files you had
    /// already edited before they were recorded have no entry.
    pub files: BTreeMap<String, String>,
}

impl Lock {
    /// Read a lockfile; a missing one is empty.
    pub fn load(path: &Path) -> Result<Lock, String> {
        match fs::read_to_string(path) {
            Ok(text) => {
                let lock: Lock =
                    serde_json::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))?;
                lock.validate()
                    .map_err(|e| format!("{}: {e}", path.display()))?;
                Ok(lock)
            }
            Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(Lock {
                version: 1,
                ..Lock::default()
            }),
            Err(e) => Err(format!("{}: {e}", path.display())),
        }
    }

    /// Refuse names and paths that would leave the project: `update` writes and deletes the
    /// files a lockfile lists, and a lockfile is just a file in the repo.
    pub fn validate(&self) -> Result<(), String> {
        for (name, item) in &self.items {
            if !crate::is_item_name(name) {
                return Err(format!("`{name}` is not a valid item name"));
            }
            if !crate::is_inside(Path::new(&item.dir)) {
                return Err(format!(
                    "`{name}`: `{}` must be a folder inside the project",
                    item.dir
                ));
            }
            if let Some(file) = item.files.keys().find(|f| !crate::is_item_file(name, f)) {
                return Err(format!(
                    "`{name}` lists `{file}`; files must be `{name}/<file name>`"
                ));
            }
        }
        Ok(())
    }

    pub fn save(&self, path: &Path) -> io::Result<()> {
        let mut s = serde_json::to_string_pretty(self).expect("lock serialises");
        s.push('\n');
        fs::write(path, s)
    }
}

/// FNV-1a, 64 bit, as 16 hex digits. Only used to tell whether a file changed.
pub fn hash(bytes: &[u8]) -> String {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in bytes {
        h ^= u64::from(*b);
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    format!("{h:016x}")
}

/// Where the pristine copies live for a given lockfile.
pub fn base_dir(lock_path: &Path) -> PathBuf {
    lock_path.parent().unwrap_or(Path::new("")).join(BASE_DIR)
}

/// The source checkout's current commit, when it is a git repository.
pub fn source_rev(root: &Path) -> Option<String> {
    let out = std::process::Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(root)
        .output()
        .ok()?;
    out.status
        .success()
        .then(|| String::from_utf8_lossy(&out.stdout).trim().to_string())
        .filter(|s| !s.is_empty())
}

fn read(path: &Path) -> io::Result<Option<Vec<u8>>> {
    match fs::read(path) {
        Ok(b) => Ok(Some(b)),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(e),
    }
}

fn write(path: &Path, bytes: &[u8]) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, bytes)
}

fn dest_of<'a>(kind: Kind, dest: &'a Path, blocks_dest: &'a Path) -> &'a Path {
    match kind {
        Kind::Component => dest,
        Kind::Block => blocks_dest,
    }
}

/// The project root: the lockfile's folder.
fn project_of(lock_path: &Path) -> &Path {
    lock_path.parent().unwrap_or(Path::new(""))
}

/// A folder as the lockfile stores it: relative to the project root when it is inside it.
fn stored(lock_path: &Path, dir: &Path) -> String {
    let project = project_of(lock_path);
    let rel = if project.as_os_str().is_empty() {
        dir
    } else {
        dir.strip_prefix(project).unwrap_or(dir)
    };
    rel.to_string_lossy().into_owned()
}

/// After `rust-ui add`: record every file of `items` whose copy in the project matches the
/// registry (just written, or already the same). A file kept because you had edited it is
/// left as the lock had it. Returns how many files were recorded.
pub fn record(
    lock_path: &Path,
    root: &Path,
    source: &str,
    items: &[Item],
    dest: &Path,
    blocks_dest: &Path,
) -> Result<usize, String> {
    let err = |e: io::Error| e.to_string();
    let mut lock = Lock::load(lock_path)?;
    lock.version = 1;
    lock.source = Some(source.to_string());
    let rev = source_rev(root);
    let bases = base_dir(lock_path);
    let mut n = 0;
    for item in items {
        let path = dest_of(item.kind, dest, blocks_dest);
        let dir = stored(lock_path, path);
        let entry = lock
            .items
            .entry(item.name.clone())
            .or_insert_with(|| LockItem {
                kind: item.kind,
                dir: dir.clone(),
                rev: None,
                files: BTreeMap::new(),
            });
        entry.kind = item.kind;
        entry.dir = dir.clone();
        entry.rev = rev.clone();
        for file in &item.files {
            let new = fs::read(root.join(item.kind.dir()).join(file)).map_err(err)?;
            if read(&path.join(file)).map_err(err)?.as_deref() == Some(&new[..]) {
                write(&bases.join(file), &new).map_err(err)?;
                entry.files.insert(file.clone(), hash(&new));
                n += 1;
            }
        }
    }
    lock.save(lock_path).map_err(err)?;
    Ok(n)
}

/// What `update` does (or would do) to one file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    /// Already the registry's version.
    UpToDate,
    /// You hadn't changed it: replaced with the registry's version.
    Updated,
    /// Both changed it in different places: merged, your edits kept.
    Merged,
    /// Both changed the same lines: written with conflict markers.
    Conflict,
    /// New in the registry (or a newly needed component): copied.
    Added,
    /// You edited it and the registry didn't change it: left alone.
    KeptEdits,
    /// Still has conflict markers from an earlier update: left alone.
    Unresolved,
    /// Differs from the registry and no copy of what was installed is recorded (it was edited
    /// before the lockfile existed): left alone.
    NoBase,
    /// You deleted it: not restored.
    DeletedLocally,
    /// The registry no longer has it: left in place, no longer tracked.
    RemovedUpstream,
}

impl Action {
    /// Short label for the command's output.
    pub fn label(self) -> &'static str {
        match self {
            Action::UpToDate => "up to date",
            Action::Updated => "updated",
            Action::Merged => "merged",
            Action::Conflict => "CONFLICT",
            Action::Added => "added",
            Action::KeptEdits => "kept",
            Action::Unresolved => "UNRESOLVED",
            Action::NoBase => "skipped",
            Action::DeletedLocally => "deleted",
            Action::RemovedUpstream => "removed",
        }
    }

    /// A note after the path in the output, if the action needs one.
    pub fn note(self) -> &'static str {
        match self {
            Action::Merged => "your edits kept",
            Action::Conflict => "both changed the same lines; resolve the <<<<<<< markers",
            Action::KeptEdits => "your edits; the registry did not change it",
            Action::Unresolved => "still has conflict markers from the last update",
            Action::NoBase => {
                "differs from the registry and no installed copy is recorded; compare with `rust-ui diff`, or take the registry's with `rust-ui add --force`"
            }
            Action::DeletedLocally => "you deleted it; not restored",
            Action::RemovedUpstream => "no longer in the registry; delete it if nothing uses it",
            _ => "",
        }
    }

    /// Needs the user to look at it.
    pub fn needs_attention(self) -> bool {
        matches!(self, Action::Conflict | Action::Unresolved | Action::NoBase)
    }
}

#[derive(Debug, Clone)]
pub struct FileUpdate {
    pub item: String,
    /// `<item>/<file>`, as in the registry.
    pub file: String,
    pub kind: Kind,
    /// The item's folder in the project (`src/components`).
    pub dir: String,
    /// The file in the project.
    pub path: PathBuf,
    pub action: Action,
    /// The project's file now.
    pub local: Option<Vec<u8>>,
    /// What the file will hold; `None` leaves it as it is.
    pub result: Option<Vec<u8>>,
    /// The registry's file now, recorded as the new base; `None` keeps (or, for
    /// [`Action::RemovedUpstream`], drops) the lock entry.
    pub new_base: Option<Vec<u8>>,
}

impl FileUpdate {
    /// Whether applying changes the file on disk.
    pub fn writes(&self) -> bool {
        self.result.is_some() && self.result != self.local
    }
}

/// Everything one `update` run does.
#[derive(Debug, Clone, Default)]
pub struct Plan {
    pub files: Vec<FileUpdate>,
    /// Items this run installs for the first time (new dependencies), with their folder.
    pub new_items: Vec<(Item, String)>,
    /// Installed items the registry no longer has.
    pub gone: Vec<String>,
    /// The source's commit (recorded per item).
    pub rev: Option<String>,
}

impl Plan {
    pub fn count(&self, action: Action) -> usize {
        self.files.iter().filter(|f| f.action == action).count()
    }

    pub fn has_conflicts(&self) -> bool {
        self.files.iter().any(|f| f.action.needs_attention())
    }
}

fn short(rev: &Option<String>, fallback: &str) -> String {
    match rev {
        Some(r) => format!("registry {}", &r[..r.len().min(7)]),
        None => fallback.to_string(),
    }
}

/// Three-way merge of one file; `Err` holds the text with conflict markers.
pub fn merge(
    base: &[u8],
    yours: &[u8],
    theirs: &[u8],
    base_label: &str,
    theirs_label: &str,
) -> Result<Vec<u8>, Vec<u8>> {
    let mut opts = diffy::MergeOptions::new();
    opts.set_conflict_style(diffy::ConflictStyle::Diff3)
        .set_incomplete_hunk_style(diffy::IncompleteHunkStyle::Git);
    opts.merge_bytes(base, yours, theirs).map_err(|text| {
        // diffy labels the sides ours / original / theirs; say which is which.
        let mut out = Vec::with_capacity(text.len() + 64);
        for line in text.split_inclusive(|b| *b == b'\n') {
            let (body, nl) = match line.strip_suffix(b"\n") {
                Some(b) => (b, &b"\n"[..]),
                None => (line, &b""[..]),
            };
            let label = match body {
                b"<<<<<<< ours" => Some(CONFLICT_START.to_string()),
                b"||||||| original" => Some(format!("||||||| {base_label}")),
                b">>>>>>> theirs" => Some(format!(">>>>>>> {theirs_label}")),
                _ => None,
            };
            match label {
                Some(l) => out.extend_from_slice(l.as_bytes()),
                None => out.extend_from_slice(body),
            }
            out.extend_from_slice(nl);
        }
        out
    })
}

fn has_markers(bytes: &[u8]) -> bool {
    bytes
        .split(|b| *b == b'\n')
        .any(|l| l == CONFLICT_START.as_bytes())
}

/// Work out what updating `names` (every installed item when empty) would do. Items the
/// updated ones now depend on but that aren't installed yet are added, into the folder of the
/// installed items of the same kind (`dest` / `blocks_dest` when there are none).
pub fn plan(
    root: &Path,
    reg: &Registry,
    lock: &Lock,
    lock_path: &Path,
    names: &[String],
    dest: &Path,
    blocks_dest: &Path,
) -> Result<Plan, String> {
    let err = |e: io::Error| e.to_string();
    let wanted: Vec<String> = if names.is_empty() {
        lock.items.keys().cloned().collect()
    } else {
        for n in names {
            if !lock.items.contains_key(n) {
                return Err(format!(
                    "`{n}` is not in {} (it wasn't installed with `rust-ui add`); run `rust-ui add {n}` first",
                    lock_path.display()
                ));
            }
        }
        names.to_vec()
    };
    let (present, gone): (Vec<String>, Vec<String>) = wanted
        .into_iter()
        .partition(|n| reg.items.iter().any(|i| &i.name == n));
    let rev = source_rev(root);
    let theirs_label = short(&rev, "registry now");
    let bases = base_dir(lock_path);
    let folder_for = |kind: Kind| {
        lock.items
            .values()
            .find(|i| i.kind == kind)
            .map(|i| i.dir.clone())
            .unwrap_or_else(|| stored(lock_path, dest_of(kind, dest, blocks_dest)))
    };

    let mut out = Plan {
        gone,
        rev: rev.clone(),
        ..Plan::default()
    };
    for item in resolve(reg, &present)? {
        let entry = lock.items.get(&item.name);
        let dir = entry
            .map(|e| e.dir.clone())
            .unwrap_or_else(|| folder_for(item.kind));
        let base_label = short(&entry.and_then(|e| e.rev.clone()), "registry when copied");
        let mut files: BTreeSet<&String> = item.files.iter().collect();
        if let Some(e) = entry {
            files.extend(e.files.keys());
        }
        let mut all_new = true;
        for file in files {
            let path = project_of(lock_path).join(&dir).join(file);
            let local = read(&path).map_err(err)?;
            let new = if item.files.contains(file) {
                Some(fs::read(root.join(item.kind.dir()).join(file)).map_err(err)?)
            } else {
                None
            };
            let recorded = entry.and_then(|e| e.files.get(file));
            let base = match recorded {
                None => None,
                Some(h) => match read(&bases.join(file)).map_err(err)? {
                    Some(b) if &hash(&b) == h => Some(b),
                    // No pristine copy (or a changed one): an untouched file is its own base.
                    _ => local.clone().filter(|l| &hash(l) == h),
                },
            };
            let (action, result) = match (&new, &local, &base) {
                (None, _, _) => (Action::RemovedUpstream, None),
                (Some(n), None, None) => (Action::Added, Some(n.clone())),
                (Some(_), None, Some(_)) => (Action::DeletedLocally, None),
                (Some(n), Some(l), _) if n == l => (Action::UpToDate, None),
                (Some(_), Some(_), None) => (Action::NoBase, None),
                (Some(n), Some(l), Some(b)) if l == b => (Action::Updated, Some(n.clone())),
                (Some(_), Some(l), Some(_)) if has_markers(l) => (Action::Unresolved, None),
                (Some(n), Some(_), Some(b)) if n == b => (Action::KeptEdits, None),
                (Some(n), Some(l), Some(b)) => match merge(b, l, n, &base_label, &theirs_label) {
                    Ok(m) => (Action::Merged, Some(m)),
                    Err(c) => (Action::Conflict, Some(c)),
                },
            };
            all_new &= action == Action::Added;
            let new_base = match action {
                Action::NoBase | Action::RemovedUpstream => None,
                _ => new,
            };
            out.files.push(FileUpdate {
                item: item.name.clone(),
                file: file.clone(),
                kind: item.kind,
                dir: dir.clone(),
                path,
                action,
                local,
                result,
                new_base,
            });
        }
        if entry.is_none() && all_new {
            out.new_items.push((item.clone(), dir));
        }
    }
    Ok(out)
}

/// Write the plan's files, the new pristine copies and the lockfile.
pub fn apply(plan: &Plan, lock_path: &Path, source: &str) -> Result<(), String> {
    let err = |e: io::Error| e.to_string();
    let mut lock = Lock::load(lock_path)?;
    lock.version = 1;
    lock.source = Some(source.to_string());
    let bases = base_dir(lock_path);
    for f in &plan.files {
        if f.writes() {
            write(&f.path, f.result.as_deref().unwrap_or_default()).map_err(err)?;
        }
        let entry = lock
            .items
            .entry(f.item.clone())
            .or_insert_with(|| LockItem {
                kind: f.kind,
                dir: f.dir.clone(),
                rev: None,
                files: BTreeMap::new(),
            });
        entry.rev = plan.rev.clone();
        match (&f.new_base, f.action) {
            (Some(new), _) => {
                write(&bases.join(&f.file), new).map_err(err)?;
                entry.files.insert(f.file.clone(), hash(new));
            }
            (None, Action::RemovedUpstream) => {
                entry.files.remove(&f.file);
                let _ = fs::remove_file(bases.join(&f.file));
            }
            (None, _) => {}
        }
    }
    lock.save(lock_path).map_err(err)
}

/// A unified diff of what applying the plan changes on disk, for `--dry-run`.
pub fn render_diff(plan: &Plan) -> String {
    let mut out = String::new();
    for f in plan.files.iter().filter(|f| f.writes()) {
        let path = f.path.to_string_lossy();
        let mut opts = diffy::DiffOptions::new();
        opts.set_original_filename(if f.local.is_some() {
            format!("a/{path}")
        } else {
            "/dev/null".into()
        })
        .set_modified_filename(format!("b/{path}"));
        let old = f.local.as_deref().unwrap_or_default();
        let new = f.result.as_deref().unwrap_or_default();
        let patch = opts.create_patch_bytes(old, new);
        let mut bytes = Vec::new();
        let _ = diffy::PatchFormatter::new().write_patch_into(&patch, &mut bytes);
        out.push_str(&String::from_utf8_lossy(&bytes));
    }
    out
}
