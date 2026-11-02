//! docs/CHECKLIST.md (the designsystemchecklist.com "Core components" scorecard) stays honest
//! about its own numbers: every section's count matches its rows, every row has a known
//! status, a row that isn't "Not yet" says how, and the totals table adds up.

use std::path::Path;

const STATUSES: [&str; 3] = ["Met", "Partly", "Not yet"];

fn checklist() -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/CHECKLIST.md");
    std::fs::read_to_string(path).expect("docs/CHECKLIST.md")
}

/// (component, declared count, rows as (item, status, how)).
type Section = (String, usize, Vec<(String, String, String)>);

fn sections(doc: &str) -> Vec<Section> {
    let mut out: Vec<Section> = Vec::new();
    for line in doc.lines() {
        if let Some(head) = line.strip_prefix("## ") {
            let (name, count) = head.rsplit_once(" (").expect("`## Name (N)`");
            let count = count.trim_end_matches(')').parse().expect("count");
            out.push((name.to_string(), count, Vec::new()));
        } else if let Some((_, _, rows)) = out.last_mut() {
            let cells: Vec<&str> = line.split('|').map(str::trim).collect();
            if cells.len() == 5 && !matches!(cells[1], "Item" | "---") {
                rows.push((cells[1].into(), cells[2].into(), cells[3].into()));
            }
        }
    }
    out
}

#[test]
fn every_section_has_its_rows_and_each_row_a_status() {
    let doc = checklist();
    let sections = sections(&doc);
    assert!(sections.len() >= 27, "{} sections", sections.len());
    for (name, count, rows) in &sections {
        assert_eq!(rows.len(), *count, "{name}: heading says {count}");
        for (item, status, how) in rows {
            assert!(
                STATUSES.contains(&status.as_str()),
                "{name} / {item}: `{status}`"
            );
            if status != "Not yet" {
                assert!(!how.is_empty(), "{name} / {item}: say how it is met");
            }
        }
    }
}

#[test]
fn the_totals_table_adds_up() {
    let doc = checklist();
    let rows: Vec<_> = sections(&doc).into_iter().flat_map(|s| s.2).collect();
    let count = |s: &str| rows.iter().filter(|r| r.1 == s).count();
    let expected = format!(
        "| All components | {} | {} | {} | {} |",
        count("Met"),
        count("Partly"),
        count("Not yet"),
        rows.len()
    );
    assert!(
        doc.contains(&expected),
        "update the totals in docs/CHECKLIST.md to:\n{expected}"
    );
}
