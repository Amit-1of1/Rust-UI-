//! Properties of `atoll update`'s three-way merge, over every shape of edit proptest can
//! build from a small alphabet of lines (a small alphabet makes edits collide often, which is
//! where merges go wrong).
//!
//! The promise `atoll update` makes is "your edits are never lost": a file either merges
//! cleanly with your changes in it, or stops with conflict markers that show both sides.

use atoll_cli::update::{self, CONFLICT_START};
use proptest::prelude::*;

/// A file: up to 8 lines from a 4-word alphabet, with or without a final newline.
fn file() -> impl Strategy<Value = Vec<u8>> {
    (
        prop::collection::vec(prop::sample::select(vec!["a", "b", "c", "d"]), 0..8),
        any::<bool>(),
    )
        .prop_map(|(lines, final_nl)| {
            let mut s = lines.join("\n");
            if final_nl && !s.is_empty() {
                s.push('\n');
            }
            s.into_bytes()
        })
}

/// An edit of `base`: each line kept, dropped or replaced, and lines inserted between them.
fn edit_of(base: Vec<u8>) -> impl Strategy<Value = (Vec<u8>, Vec<u8>)> {
    let n = lines(&base).len();
    (
        prop::collection::vec(0u8..4, n),
        prop::collection::vec(
            prop::option::of(prop::sample::select(vec!["x", "y", "a"])),
            n + 1,
        ),
    )
        .prop_map(move |(ops, inserts)| {
            let ls = lines(&base);
            let mut out: Vec<String> = Vec::new();
            for (i, l) in ls.iter().enumerate() {
                if let Some(ins) = inserts[i] {
                    out.push(ins.to_string());
                }
                match ops[i] {
                    0 | 1 => out.push(l.clone()),
                    2 => {}
                    _ => out.push(format!("{l}{l}")),
                }
            }
            if let Some(ins) = inserts[ls.len()] {
                out.push(ins.to_string());
            }
            let mut s = out.join("\n");
            if base.ends_with(b"\n") && !s.is_empty() {
                s.push('\n');
            }
            (base.clone(), s.into_bytes())
        })
}

fn lines(b: &[u8]) -> Vec<String> {
    String::from_utf8_lossy(b)
        .lines()
        .map(str::to_string)
        .collect()
}

/// Lines of `side` that `base` doesn't have as many of (a multiset difference): what that side
/// added.
fn added(base: &[u8], side: &[u8]) -> Vec<String> {
    let mut pool = lines(base);
    let mut out = Vec::new();
    for l in lines(side) {
        match pool.iter().position(|p| *p == l) {
            Some(i) => {
                pool.remove(i);
            }
            None => out.push(l),
        }
    }
    out
}

fn contains_all(text: &[u8], want: &[String]) -> Result<(), String> {
    let mut pool = lines(text);
    for l in want {
        match pool.iter().position(|p| p == l) {
            Some(i) => {
                pool.remove(i);
            }
            None => return Err(format!("line {l:?} is missing")),
        }
    }
    Ok(())
}

fn merge(b: &[u8], y: &[u8], t: &[u8]) -> Result<Vec<u8>, Vec<u8>> {
    update::merge(b, y, t, "base", "new")
}

fn show(b: &[u8]) -> String {
    String::from_utf8_lossy(b).into_owned()
}

/// A base and two independent edits of it.
fn three_way() -> impl Strategy<Value = (Vec<u8>, Vec<u8>, Vec<u8>)> {
    file()
        .prop_flat_map(|base| (edit_of(base.clone()), edit_of(base)))
        .prop_map(|((base, yours), (_, theirs))| (base, yours, theirs))
}

proptest! {
    #![proptest_config(ProptestConfig { cases: 4096, ..ProptestConfig::default() })]

    /// The registry didn't change: the file is exactly yours.
    #[test]
    fn unchanged_upstream_keeps_yours((base, yours) in file().prop_flat_map(edit_of)) {
        prop_assert_eq!(merge(&base, &yours, &base), Ok(yours));
    }

    /// You didn't edit: the file is exactly the registry's.
    #[test]
    fn untouched_file_takes_theirs((base, theirs) in file().prop_flat_map(edit_of)) {
        prop_assert_eq!(merge(&base, &base, &theirs), Ok(theirs));
    }

    /// Both sides made the same change: no conflict.
    #[test]
    fn same_change_is_not_a_conflict((base, both) in file().prop_flat_map(edit_of)) {
        prop_assert_eq!(merge(&base, &both, &both), Ok(both));
    }

    /// Never lose an edit. A clean merge contains every line either side added; a conflict
    /// contains both sides in full, between markers that `atoll update` recognises.
    #[test]
    fn no_edit_is_lost((base, yours, theirs) in three_way()) {
        match merge(&base, &yours, &theirs) {
            Ok(m) => {
                let mine = added(&base, &yours);
                let new = added(&base, &theirs);
                if let Err(e) = contains_all(&m, &mine) {
                    return Err(TestCaseError::fail(format!(
                        "your edit was lost: {e}\nbase {:?}\nyours {:?}\ntheirs {:?}\nmerged {:?}",
                        show(&base), show(&yours), show(&theirs), show(&m))));
                }
                if let Err(e) = contains_all(&m, &new) {
                    return Err(TestCaseError::fail(format!(
                        "the registry's edit was lost: {e}\nbase {:?}\nyours {:?}\ntheirs {:?}\nmerged {:?}",
                        show(&base), show(&yours), show(&theirs), show(&m))));
                }
                prop_assert!(!lines(&m).iter().any(|l| l == CONFLICT_START));
            }
            Err(text) => {
                prop_assert!(lines(&text).iter().any(|l| l == CONFLICT_START),
                    "a conflict without a marker: {:?}", show(&text));
                // Taking "yours" in every conflict keeps every line you added, and taking the
                // registry's side keeps every line it added: the markers hide nothing.
                let y = pick(&text, Side::Yours).join("\n").into_bytes();
                let t = pick(&text, Side::Theirs).join("\n").into_bytes();
                prop_assert!(contains_all(&y, &added(&base, &yours)).is_ok(),
                    "your side lost a line: {:?}", show(&text));
                prop_assert!(contains_all(&t, &added(&base, &theirs)).is_ok(),
                    "the registry's side lost a line: {:?}", show(&text));
            }
        }
    }

    /// Merging is symmetric up to the sides' labels: swapping the two edits doesn't turn a clean
    /// merge into a conflict or change the result.
    #[test]
    fn clean_merges_are_symmetric((base, yours, theirs) in three_way()) {
        let a = merge(&base, &yours, &theirs);
        let b = merge(&base, &theirs, &yours);
        prop_assert_eq!(a.is_ok(), b.is_ok());
        if let (Ok(a), Ok(b)) = (a, b) {
            prop_assert_eq!(a, b);
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Side {
    Yours,
    Theirs,
}

/// Resolve every conflict in `text` by taking one side, dropping markers and the base section.
fn pick(text: &[u8], side: Side) -> Vec<String> {
    #[derive(PartialEq)]
    enum In {
        Out,
        Yours,
        Base,
        Theirs,
    }
    let mut at = In::Out;
    let mut out = Vec::new();
    for l in lines(text) {
        if l == CONFLICT_START {
            at = In::Yours;
        } else if at == In::Yours && l.starts_with("||||||| ") {
            at = In::Base;
        } else if (at == In::Yours || at == In::Base) && l == "=======" {
            at = In::Theirs;
        } else if at == In::Theirs && l.starts_with(">>>>>>> ") {
            at = In::Out;
        } else {
            let keep = match at {
                In::Out => true,
                In::Yours => side == Side::Yours,
                In::Base => false,
                In::Theirs => side == Side::Theirs,
            };
            if keep {
                out.push(l);
            }
        }
    }
    out
}

/// Every file of up to 4 lines over {a, b} (31 files), as base, yours and theirs: all 29,791
/// triples. Proptest samples; this proves the promises for every small file, which is where
/// diff3's edge cases (empty files, repeated lines, edits at both ends) live.
#[test]
fn every_small_triple_keeps_every_edit() {
    let mut files: Vec<Vec<u8>> = Vec::new();
    for len in 0..=4u32 {
        for bits in 0..(1u32 << len) {
            let mut s = String::new();
            for i in 0..len {
                s.push(if bits >> i & 1 == 1 { 'b' } else { 'a' });
                s.push('\n');
            }
            files.push(s.into_bytes());
        }
    }
    assert_eq!(files.len(), 31);
    let (mut clean, mut conflicts) = (0u32, 0u32);
    for base in &files {
        for yours in &files {
            for theirs in &files {
                let ctx = || {
                    format!(
                        "base {:?} yours {:?} theirs {:?}",
                        show(base),
                        show(yours),
                        show(theirs)
                    )
                };
                let got = merge(base, yours, theirs);
                if theirs == base || yours == theirs {
                    assert_eq!(got, Ok(yours.clone()), "{}", ctx());
                }
                if yours == base {
                    assert_eq!(got, Ok(theirs.clone()), "{}", ctx());
                }
                match got {
                    Ok(m) => {
                        clean += 1;
                        contains_all(&m, &added(base, yours)).unwrap_or_else(|e| {
                            panic!("your edit was lost ({e}): {} merged {:?}", ctx(), show(&m))
                        });
                        contains_all(&m, &added(base, theirs)).unwrap_or_else(|e| {
                            panic!(
                                "the registry's edit was lost ({e}): {} merged {:?}",
                                ctx(),
                                show(&m)
                            )
                        });
                    }
                    Err(text) => {
                        conflicts += 1;
                        assert!(
                            lines(&text).iter().any(|l| l == CONFLICT_START),
                            "{}",
                            ctx()
                        );
                        let y = pick(&text, Side::Yours).join("\n").into_bytes();
                        let t = pick(&text, Side::Theirs).join("\n").into_bytes();
                        assert!(contains_all(&y, &added(base, yours)).is_ok(), "{}", ctx());
                        assert!(contains_all(&t, &added(base, theirs)).is_ok(), "{}", ctx());
                    }
                }
            }
        }
    }
    assert_eq!(clean + conflicts, 29_791);
    println!("{clean} clean merges, {conflicts} conflicts, no edit lost");
}
