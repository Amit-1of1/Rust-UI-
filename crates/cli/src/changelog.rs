//! `atoll changelog`: fold the per-PR fragments in `changes/` into CHANGELOG.md and
//! docs/MIGRATION.md.
//!
//! Every branch used to add its lines to the top of the same sections of CHANGELOG.md and
//! MIGRATION.md, so every pair of open PRs conflicted there. Now a branch adds its own
//! `changes/<topic>.md` (and `changes/<topic>.migration.md` for a breaking change) and never
//! edits the two shared files; folding happens on `main` before a release (changes/README.md).

use std::fs;
use std::path::{Path, PathBuf};

pub const CHANGES_DIR: &str = "changes";
pub const CHANGELOG: &str = "CHANGELOG.md";
pub const MIGRATION: &str = "docs/MIGRATION.md";

/// Changelog sections in the order they appear under "Unreleased" (Keep a Changelog, with
/// Breaking first because it is what readers look for).
pub const SECTIONS: &[&str] = &[
    "Breaking",
    "Added",
    "Changed",
    "Deprecated",
    "Removed",
    "Fixed",
    "Security",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fragment {
    pub path: PathBuf,
    /// The highest `#N` in the file, used to put newer PRs first.
    pub pr: Option<u32>,
    /// `(section, entries)` with the entries' text as written (bullets, trimmed).
    pub sections: Vec<(String, String)>,
    /// The matching `<topic>.migration.md`, verbatim, if there is one.
    pub migration: Option<String>,
}

/// Highest PR number written as `#123` in `text`.
fn highest_pr(text: &str) -> Option<u32> {
    let bytes = text.as_bytes();
    let mut best = None;
    for (i, _) in text.match_indices('#') {
        let digits: String = bytes[i + 1..]
            .iter()
            .take_while(|b| b.is_ascii_digit())
            .map(|&b| b as char)
            .collect();
        if let Ok(n) = digits.parse::<u32>() {
            best = best.max(Some(n));
        }
    }
    best
}

/// Split a fragment into its `### Section` blocks. Errors name the file and the problem.
pub fn parse(path: &Path, text: &str) -> Result<Vec<(String, String)>, String> {
    let name = path.display();
    let mut out: Vec<(String, String)> = Vec::new();
    for line in text.lines() {
        if let Some(head) = line.strip_prefix("### ") {
            let head = head.trim();
            if !SECTIONS.contains(&head) {
                return Err(format!(
                    "{name}: unknown section `### {head}` (use one of {})",
                    SECTIONS.join(", ")
                ));
            }
            if out.iter().any(|(s, _)| s == head) {
                return Err(format!("{name}: `### {head}` appears twice"));
            }
            out.push((head.to_string(), String::new()));
        } else if line.starts_with('#') {
            return Err(format!(
                "{name}: only `### <section>` headings belong in a fragment, found `{line}`"
            ));
        } else if let Some((_, body)) = out.last_mut() {
            body.push_str(line);
            body.push('\n');
        } else if !line.trim().is_empty() {
            return Err(format!(
                "{name}: text before the first `### <section>` heading"
            ));
        }
    }
    for (head, body) in &mut out {
        *body = body.trim().to_string();
        if body.is_empty() {
            return Err(format!("{name}: `### {head}` has no entries"));
        }
    }
    if out.is_empty() {
        return Err(format!("{name}: no `### <section>` headings"));
    }
    Ok(out)
}

/// Every fragment in `root/changes`, newest PR first. `README.md` is not a fragment.
pub fn load(root: &Path) -> Result<Vec<Fragment>, String> {
    let dir = root.join(CHANGES_DIR);
    let Ok(entries) = fs::read_dir(&dir) else {
        return Ok(Vec::new());
    };
    let mut paths: Vec<PathBuf> = entries
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|e| e == "md"))
        .filter(|p| !p.ends_with("README.md"))
        .collect();
    paths.sort();
    let mut out = Vec::new();
    for path in &paths {
        let file = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        if let Some(topic) = file.strip_suffix(".migration.md") {
            if !dir.join(format!("{topic}.md")).is_file() {
                return Err(format!(
                    "{}: no changes/{topic}.md beside it (a migration note needs a Breaking entry)",
                    path.display()
                ));
            }
            continue;
        }
        let text = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
        let sections = parse(path, &text)?;
        let topic = file.trim_end_matches(".md");
        let mig_path = dir.join(format!("{topic}.migration.md"));
        let migration = match fs::read_to_string(&mig_path) {
            Ok(m) => {
                if !m.trim_start().starts_with("### ") {
                    return Err(format!(
                        "{}: start with a `### <what changed> (PR #N)` heading",
                        mig_path.display()
                    ));
                }
                Some(m.trim().to_string())
            }
            Err(_) => None,
        };
        out.push(Fragment {
            path: path.clone(),
            pr: highest_pr(&text),
            sections,
            migration,
        });
    }
    // Newest first, like the rest of the changelog; fragments without a number go on top.
    out.sort_by(|a, b| b.pr.unwrap_or(u32::MAX).cmp(&a.pr.unwrap_or(u32::MAX)));
    Ok(out)
}

/// Fragments that still have no PR number (`(#N)`), which `fold` would publish unnamed.
pub fn unnamed(fragments: &[Fragment]) -> Vec<&Path> {
    fragments
        .iter()
        .filter(|f| f.pr.is_none())
        .map(|f| f.path.as_path())
        .collect()
}

/// The CHANGELOG with every fragment's entries added to the top of its "Unreleased" sections.
pub fn fold_changelog(changelog: &str, fragments: &[Fragment]) -> Result<String, String> {
    let lines: Vec<&str> = changelog.lines().collect();
    let start = lines
        .iter()
        .position(|l| l.trim() == "## Unreleased")
        .ok_or("CHANGELOG.md has no `## Unreleased` heading")?;
    let end = lines[start + 1..]
        .iter()
        .position(|l| l.starts_with("## "))
        .map(|i| start + 1 + i)
        .unwrap_or(lines.len());

    // Existing Unreleased sections: name -> body lines.
    let mut existing: Vec<(String, Vec<String>)> = Vec::new();
    let mut preamble: Vec<String> = Vec::new();
    for line in &lines[start + 1..end] {
        if let Some(head) = line.strip_prefix("### ") {
            existing.push((head.trim().to_string(), Vec::new()));
        } else if let Some((_, body)) = existing.last_mut() {
            body.push(line.to_string());
        } else {
            preamble.push(line.to_string());
        }
    }

    let mut order: Vec<String> = SECTIONS.iter().map(|s| s.to_string()).collect();
    for (name, _) in &existing {
        if !order.contains(name) {
            order.push(name.clone());
        }
    }
    let mut section_text = String::new();
    for name in &order {
        let new: Vec<&str> = fragments
            .iter()
            .flat_map(|f| f.sections.iter())
            .filter(|(s, _)| s == name)
            .map(|(_, body)| body.as_str())
            .collect();
        let old = existing
            .iter()
            .find(|(s, _)| s == name)
            .map(|(_, b)| b.join("\n").trim().to_string())
            .unwrap_or_default();
        if new.is_empty() && old.is_empty() {
            continue;
        }
        section_text.push_str(&format!("### {name}\n\n"));
        for body in new {
            section_text.push_str(body);
            section_text.push('\n');
        }
        if !old.is_empty() {
            section_text.push_str(&old);
            section_text.push('\n');
        }
        section_text.push('\n');
    }

    let mut out = lines[..=start].join("\n");
    out.push('\n');
    let pre = preamble.join("\n");
    let pre = pre.trim();
    out.push('\n');
    if !pre.is_empty() {
        out.push_str(pre);
        out.push_str("\n\n");
    }
    out.push_str(&section_text);
    if end < lines.len() {
        out.push_str(&lines[end..].join("\n"));
        out.push('\n');
    }
    let out = out.trim_end().to_string() + "\n";
    Ok(out)
}

/// MIGRATION.md with every fragment's migration note added to the top of "Unreleased".
pub fn fold_migration(migration: &str, fragments: &[Fragment]) -> Result<String, String> {
    let notes: Vec<&str> = fragments
        .iter()
        .filter_map(|f| f.migration.as_deref())
        .collect();
    if notes.is_empty() {
        return Ok(migration.to_string());
    }
    let at = migration
        .find("## Unreleased\n")
        .ok_or("docs/MIGRATION.md has no `## Unreleased` heading")?
        + "## Unreleased\n".len();
    let (head, rest) = migration.split_at(at);
    Ok(format!(
        "{head}\n{}\n\n{}",
        notes.join("\n\n"),
        rest.trim_start_matches('\n')
    ))
}

/// Fold every fragment into the two files and delete the fragments. Returns the fragments.
pub fn fold(root: &Path) -> Result<Vec<Fragment>, String> {
    let fragments = load(root)?;
    let missing = unnamed(&fragments);
    if !missing.is_empty() {
        return Err(format!(
            "these fragments name no PR (`(#N)`); add the number first: {}",
            missing
                .iter()
                .map(|p| p.display().to_string())
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }
    if fragments.is_empty() {
        return Ok(fragments);
    }
    let read = |p: &str| fs::read_to_string(root.join(p)).map_err(|e| format!("{p}: {e}"));
    let changelog = fold_changelog(&read(CHANGELOG)?, &fragments)?;
    let migration = fold_migration(&read(MIGRATION)?, &fragments)?;
    let write = |p: &str, s: &str| fs::write(root.join(p), s).map_err(|e| format!("{p}: {e}"));
    write(CHANGELOG, &changelog)?;
    write(MIGRATION, &migration)?;
    for f in &fragments {
        fs::remove_file(&f.path).map_err(|e| format!("{}: {e}", f.path.display()))?;
        let topic = f
            .path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .trim_end_matches(".md");
        let mig = root.join(CHANGES_DIR).join(format!("{topic}.migration.md"));
        if mig.is_file() {
            fs::remove_file(&mig).map_err(|e| format!("{}: {e}", mig.display()))?;
        }
    }
    Ok(fragments)
}
