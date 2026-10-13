//! `Selection` must re-render only the rows whose state changes.

use dioxus::prelude::*;
use std::cell::{Cell, RefCell};
use ui::prelude::*;

thread_local! {
    static RENDERS: RefCell<Vec<usize>> = const { RefCell::new(Vec::new()) };
    static SELECTION: Cell<Option<Selection<usize>>> = const { Cell::new(None) };
}

fn renders() -> Vec<usize> {
    RENDERS.with(|r| std::mem::take(&mut *r.borrow_mut()))
}

fn selection() -> Selection<usize> {
    SELECTION.with(|s| s.get()).expect("list rendered")
}

#[component]
fn Row(id: usize, selection: Selection<usize>) -> Element {
    let selected = use_selected(selection, id);
    RENDERS.with(|r| r.borrow_mut().push(id));
    rsx! { li { "data-selected": selected(), "{id}" } }
}

#[component]
fn List() -> Element {
    let selection = use_selection::<usize>();
    SELECTION.with(|s| s.set(Some(selection)));
    rsx! {
        ul {
            for id in 0..100usize {
                Row { key: "{id}", id, selection }
            }
        }
    }
}

fn flush(dom: &mut VirtualDom) -> String {
    dom.render_immediate(&mut dioxus_core::NoOpMutations);
    dioxus_ssr::render(dom)
}

#[test]
fn only_changed_rows_render() {
    let mut dom = VirtualDom::new(List);
    dom.rebuild_in_place();
    assert_eq!(renders().len(), 100);

    dom.in_runtime(|| selection().select_only(7));
    let html = flush(&mut dom);
    assert_eq!(renders(), vec![7]);
    assert!(html.contains("data-selected=true>7<"), "{html}");

    // Moving the selection flips exactly the old and the new row.
    dom.in_runtime(|| selection().select_only(42));
    flush(&mut dom);
    let mut r = renders();
    r.sort();
    assert_eq!(r, vec![7, 42]);

    // Selecting what is already selected does nothing.
    dom.in_runtime(|| selection().select_only(42));
    flush(&mut dom);
    assert!(renders().is_empty());

    dom.in_runtime(|| {
        let s = selection();
        s.toggle(3);
        assert!(s.contains(&3) && s.contains(&42));
        assert_eq!(s.keys().len(), 2);
    });
    flush(&mut dom);
    assert_eq!(renders(), vec![3]);

    dom.in_runtime(|| selection().clear());
    flush(&mut dom);
    let mut r = renders();
    r.sort();
    assert_eq!(r, vec![3, 42]);
    dom.in_runtime(|| assert!(selection().keys().is_empty()));
}
