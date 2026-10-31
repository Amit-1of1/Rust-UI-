//! `atoll acr`: a draft Accessibility Conformance Report (the VPAT 2.5 WCAG layout) for the
//! kit components an app uses.
//!
//! Teams that sell software to governments, universities and large companies are asked for an
//! ACR, and most fill it in by hand from an audit. Much of what goes into it for a component
//! library is already proven by this repo's tests, so the report is generated from them:
//! `docs/acr/wcag22.toml` says, for each WCAG 2.2 A and AA criterion, what the components do
//! and which checks show it, and the keyboard rows come from each used component's
//! `contract.toml`, gaps included. What only the app can decide is marked "review" with what to
//! check. The output is a starting point for the app's own ACR, not an audit.

use crate::contract::Contract;
use crate::{Kind, Registry, COMPONENTS_DIR};
use serde::Deserialize;
use std::fs;
use std::path::Path;

pub const CRITERIA_FILE: &str = "docs/acr/wcag22.toml";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Status {
    Supports,
    Partial,
    DoesNotSupport,
    NotApplicable,
    NotEvaluated,
}

impl Status {
    /// The VPAT term.
    pub fn label(self) -> &'static str {
        match self {
            Status::Supports => "Supports",
            Status::Partial => "Partially Supports",
            Status::DoesNotSupport => "Does Not Support",
            Status::NotApplicable => "Not Applicable",
            Status::NotEvaluated => "Not Evaluated",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Computed {
    /// Lists each used component's keyboard contract and its gaps.
    Keyboard,
    /// Lists the contract gaps that are about where focus goes.
    Focus,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Criterion {
    pub id: String,
    pub name: String,
    pub level: String,
    pub kit: Status,
    pub remarks: String,
    #[serde(default)]
    pub app: Option<String>,
    pub evidence: Vec<String>,
    #[serde(default)]
    pub computed: Option<Computed>,
    /// The only components the criterion is about; when the app uses none of them the row is
    /// Not Applicable for the kit.
    #[serde(default)]
    pub components: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct File {
    criterion: Vec<Criterion>,
}

pub fn load_criteria(root: &Path) -> Result<Vec<Criterion>, String> {
    let path = root.join(CRITERIA_FILE);
    let text = fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
    let mut list = toml::from_str::<File>(&text)
        .map_err(|e| format!("{}: {e}", path.display()))?
        .criterion;
    list.sort_by_key(|c| {
        c.id.split('.')
            .map(|n| n.parse::<u32>().unwrap_or(0))
            .collect::<Vec<_>>()
    });
    Ok(list)
}

/// Evidence entries that point at nothing: a missing file, or `file::test` where the file has
/// no `fn test`.
pub fn missing_evidence(root: &Path, criteria: &[Criterion]) -> Vec<String> {
    let mut out = Vec::new();
    for c in criteria {
        for e in &c.evidence {
            let (file, test) = match e.split_once("::") {
                Some((f, t)) => (f, Some(t)),
                None => (e.as_str(), None),
            };
            match fs::read_to_string(root.join(file)) {
                Err(_) => out.push(format!("{}: {file} does not exist", c.id)),
                Ok(src) => {
                    if let Some(t) = test {
                        if !src.contains(&format!("fn {t}(")) {
                            out.push(format!("{}: {file} has no test `{t}`", c.id));
                        }
                    }
                }
            }
        }
    }
    out
}

/// One row of the report.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Row {
    pub id: String,
    pub name: String,
    pub level: String,
    pub status: Status,
    /// The app has something to check for this criterion.
    pub review: bool,
    pub remarks: String,
    pub evidence: Vec<String>,
}

pub struct Report {
    pub product: String,
    pub components: Vec<String>,
    pub rows: Vec<Row>,
}

/// Builds the rows for `components` (component folder names).
pub fn report(
    root: &Path,
    criteria: &[Criterion],
    components: &[String],
    product: &str,
) -> Result<Report, String> {
    let dir = root.join(COMPONENTS_DIR);
    let mut contracts = Vec::new();
    let mut uncontracted = Vec::new();
    for name in components {
        match Contract::load(&dir, name)? {
            Some(c) => contracts.push(c),
            None if keyboard_operable(root, name) => uncontracted.push(name.clone()),
            None => {}
        }
    }
    let mut rows = Vec::new();
    for c in criteria {
        let mut status = c.kit;
        let mut remarks = c.remarks.clone();
        let mut evidence = c.evidence.clone();
        if !c.components.is_empty() && !c.components.iter().any(|n| components.contains(n)) {
            status = Status::NotApplicable;
            remarks = format!(
                "None of the components this is about ({}) are used.",
                c.components.join(", ")
            );
            evidence.clear();
        }
        if let Some(kind) = c.computed {
            let gaps: Vec<String> = contracts
                .iter()
                .flat_map(|k| {
                    k.gaps
                        .iter()
                        .filter(|g| kind == Computed::Keyboard || mentions_focus(g))
                        .map(move |g| format!("{}: {g}", k.name))
                })
                .collect();
            if !contracts.is_empty() {
                let names: Vec<&str> = contracts.iter().map(|k| k.name.as_str()).collect();
                remarks.push_str(&format!(
                    " Components with a contract: {}.",
                    names.join(", ")
                ));
                evidence.extend(
                    contracts
                        .iter()
                        .map(|k| format!("{COMPONENTS_DIR}/{}/contract.toml", k.name)),
                );
            }
            if !gaps.is_empty() {
                status = worse(status, Status::Partial);
                remarks.push_str(&format!(" Known gaps: {}", gaps.join(" ")));
            }
            if kind == Computed::Keyboard && !uncontracted.is_empty() {
                status = worse(status, Status::Partial);
                remarks.push_str(&format!(
                    " Keyboard-operable without a contract yet (covered only by the browser keyboard and axe checks): {}.",
                    uncontracted.join(", ")
                ));
            }
        }
        if let Some(app) = &c.app {
            remarks.push_str(&format!(" **Your app:** {app}"));
        }
        rows.push(Row {
            id: c.id.clone(),
            name: c.name.clone(),
            level: c.level.clone(),
            status,
            review: c.app.is_some(),
            remarks,
            evidence,
        });
    }
    Ok(Report {
        product: product.to_string(),
        components: components.to_vec(),
        rows,
    })
}

fn worse(a: Status, b: Status) -> Status {
    let rank = |s: Status| match s {
        Status::Supports => 0,
        Status::Partial => 1,
        Status::DoesNotSupport => 2,
        Status::NotApplicable | Status::NotEvaluated => 0,
    };
    if rank(b) > rank(a) {
        b
    } else {
        a
    }
}

fn mentions_focus(gap: &str) -> bool {
    let g = gap.to_lowercase();
    g.contains("focus")
}

/// Whether a component without a contract has keyboard behaviour (the same markup test as
/// `atoll doctor`).
fn keyboard_operable(root: &Path, name: &str) -> bool {
    let snapshot = fs::read_to_string(root.join(format!("crates/ui/tests/snapshots/{name}.html")))
        .unwrap_or_default();
    crate::doctor::INTERACTIVE
        .iter()
        .any(|m| snapshot.contains(m))
}

/// The components behind a list of registry item names: blocks become the components they
/// use, and every component brings its dependencies.
pub fn components_of(reg: &Registry, names: &[String]) -> Result<Vec<String>, String> {
    let items = crate::resolve(reg, names)?;
    let mut out: Vec<String> = items
        .into_iter()
        .filter(|i| i.kind == Kind::Component)
        .map(|i| i.name)
        .collect();
    out.sort();
    out.dedup();
    Ok(out)
}

fn cell(s: &str) -> String {
    s.replace('|', "\\|").replace('\n', " ")
}

/// The report as Markdown, in the VPAT 2.5 (WCAG edition) table layout.
pub fn markdown(r: &Report, date: &str) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "# {} Accessibility Conformance Report (draft)\n\n",
        r.product
    ));
    out.push_str(
        "Based on the VPAT® 2.5 WCAG edition layout. Standard: WCAG 2.2, Levels A and AA.\n\n",
    );
    out.push_str(&format!("- **Report date:** {date}\n"));
    out.push_str(&format!(
        "- **Generated by:** `atoll acr` (atoll-cli {}), from the Atoll kit's automated checks\n",
        crate::VERSION
    ));
    out.push_str(&format!(
        "- **Kit components covered ({}):** {}\n\n",
        r.components.len(),
        r.components.join(", ")
    ));
    out.push_str(
        "> **This is a draft, not an audit.** Each row says what the kit's components do and \
         which automated check shows it. Rows marked *review* depend on how your app uses \
         them, and the app's own content, pages and custom code are not covered at all. \
         Check every row, test with a screen reader (docs/SCREEN_READERS.md), and edit this \
         file before you hand it to anyone.\n\n",
    );
    let counts = |level: &str| {
        let rows: Vec<&Row> = r.rows.iter().filter(|x| x.level == level).collect();
        let n = |s: Status| rows.iter().filter(|x| x.status == s).count();
        format!(
            "{} criteria: {} supports, {} partially supports, {} not applicable, {} not evaluated, {} need your review",
            rows.len(),
            n(Status::Supports),
            n(Status::Partial),
            n(Status::NotApplicable),
            n(Status::NotEvaluated),
            rows.iter().filter(|x| x.review).count()
        )
    };
    for (title, level) in [
        ("Table 1: Success Criteria, Level A", "A"),
        ("Table 2: Success Criteria, Level AA", "AA"),
    ] {
        out.push_str(&format!("## {title}\n\n{}\n\n", counts(level)));
        out.push_str("| Criteria | Conformance Level | Remarks and Explanations |\n");
        out.push_str("| --- | --- | --- |\n");
        for row in r.rows.iter().filter(|x| x.level == level) {
            let status = if row.review {
                format!("{} (kit) · review", row.status.label())
            } else {
                row.status.label().to_string()
            };
            out.push_str(&format!(
                "| {} {} (Level {}) | {} | {} |\n",
                row.id,
                cell(&row.name),
                row.level,
                status,
                cell(&row.remarks)
            ));
        }
        out.push('\n');
    }
    out.push_str("## Evidence\n\nThe checks behind each row, in the Atoll repository:\n\n");
    for row in r.rows.iter().filter(|x| !x.evidence.is_empty()) {
        out.push_str(&format!(
            "- **{}:** {}\n",
            row.id,
            row.evidence
                .iter()
                .map(|e| format!("`{e}`"))
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }
    out.push_str(
        "\n## Terms\n\n- **Supports:** the kit's components meet the criterion without known defects.\n\
         - **Partially Supports:** some functionality does not meet it; the remarks say which.\n\
         - **Not Applicable:** the kit has nothing the criterion is about; the app may.\n\
         - **Not Evaluated:** the kit has no check for it yet; evaluate it in your app.\n\
         - **(kit) · review:** the kit's part is as stated; what your app must still do follows \"Your app\".\n",
    );
    out
}

/// Today's date (UTC) as YYYY-MM-DD.
pub fn today() -> String {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_secs());
    civil(secs / 86_400)
}

/// Days since 1970-01-01 to a calendar date (Howard Hinnant's `civil_from_days`).
fn civil(days: u64) -> String {
    let z = days as i64 + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = yoe + era * 400 + i64::from(m <= 2);
    format!("{y:04}-{m:02}-{d:02}")
}

#[cfg(test)]
mod tests {
    #[test]
    fn civil_dates() {
        assert_eq!(super::civil(0), "1970-01-01");
        assert_eq!(super::civil(20_756), "2026-10-30");
        assert_eq!(super::civil(11_016), "2000-02-29");
    }
}
