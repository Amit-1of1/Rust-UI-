//! Markup snapshots: every demo's server-rendered HTML is stored in `tests/snapshots/`,
//! one tag per line. Any change to structure or ARIA then shows up as a readable diff in
//! review, which is how we keep the guardrail "never change a primitive's markup when
//! restyling" honest.
//!
//! After an intended change, refresh them with
//! `UPDATE_SNAPSHOTS=1 cargo test -p rust-ui-kit --test snapshots` and review the diff.

use common::{render, DEMOS};

mod common;

const DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/snapshots");

/// Demos whose markup depends on today's date, so a snapshot would change every day.
const CLOCK_DEPENDENT: &[&str] = &["calendar", "date_picker", "date_time_picker"];

/// Generated ids (`dxc-12`, `dx-carousel-3`, `span-dxc-27-label`) come from process-wide
/// counters, so they depend on what else rendered first. Renumber every number that
/// follows a word and a dash, in order of first appearance per prefix.
///
/// CSS-module class names (`dx-button-3c19f381`) end in a hash of the stylesheet's absolute
/// path, so they differ between checkouts; that hash becomes `css`.
fn normalize_ids(html: &str) -> String {
    let is_word = |c: char| c.is_ascii_alphanumeric() || c == '-' || c == '_';
    let mut seen: Vec<(String, String)> = Vec::new();
    let mut out = String::with_capacity(html.len());
    let mut rest = html;
    while !rest.is_empty() {
        let end = rest.find(|c: char| !is_word(c)).unwrap_or(rest.len());
        let (token, tail) = rest.split_at(end);
        let mut parts: Vec<String> = token.split('-').map(str::to_string).collect();
        for i in 1..parts.len() {
            // Eight hex digits, which can be all letters (`bebfbacb`).
            let hash =
                parts[i].len() == 8 && parts[i].chars().all(|c| matches!(c, '0'..='9' | 'a'..='f'));
            if hash && parts[i - 1].chars().any(|c| c.is_ascii_alphabetic()) {
                parts[i] = "css".to_string();
                continue;
            }
            let numeric = !parts[i].is_empty() && parts[i].chars().all(|c| c.is_ascii_digit());
            if numeric && parts[i - 1].chars().any(|c| c.is_ascii_alphabetic()) {
                let prefix = parts[..i].join("-");
                let key = (prefix.clone(), parts[i].clone());
                let n = match seen.iter().position(|k| *k == key) {
                    Some(n) => seen[..n].iter().filter(|k| k.0 == prefix).count(),
                    None => {
                        let n = seen.iter().filter(|k| k.0 == prefix).count();
                        seen.push(key);
                        n
                    }
                };
                parts[i] = format!("n{n}");
            }
        }
        out.push_str(&parts.join("-"));
        let skip = tail.find(is_word).unwrap_or(tail.len());
        out.push_str(&tail[..skip]);
        rest = &tail[skip..];
    }
    out
}

/// One tag per line, so a diff points at the element that changed.
fn pretty(html: &str) -> String {
    let html = normalize_ids(html);
    let mut out = String::with_capacity(html.len() + html.len() / 8);
    for (i, part) in html.split('<').enumerate() {
        if i > 0 {
            out.push('\n');
            out.push('<');
        }
        out.push_str(part);
    }
    out.trim_start().to_string() + "\n"
}

#[test]
fn demo_markup_matches_snapshots() {
    let update = std::env::var_os("UPDATE_SNAPSHOTS").is_some();
    let mut stale = Vec::new();
    for (name, demo) in DEMOS {
        if CLOCK_DEPENDENT.contains(name) {
            continue;
        }
        let path = format!("{DIR}/{name}.html");
        let fresh = pretty(&render(*demo));
        let stored = std::fs::read_to_string(&path).unwrap_or_default();
        if stored == fresh {
            continue;
        }
        if update {
            std::fs::create_dir_all(DIR).expect("create snapshots dir");
            std::fs::write(&path, fresh).expect("write snapshot");
        } else {
            stale.push(*name);
        }
    }
    assert!(
        stale.is_empty(),
        "markup changed for {stale:?}; if intended, rerun with UPDATE_SNAPSHOTS=1 and review the diff"
    );
}

#[test]
fn snapshots_are_stable_between_renders() {
    for (name, demo) in DEMOS {
        if !CLOCK_DEPENDENT.contains(name) {
            assert_eq!(
                pretty(&render(*demo)),
                pretty(&render(*demo)),
                "{name} renders differently each time"
            );
        }
    }
}

#[test]
fn every_demo_file_is_snapshotted() {
    common::assert_demos_complete();
}
