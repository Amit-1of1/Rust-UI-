//! The TLA+ model of `MotionValue` (proofs/tla/MotionValue.tla) checks the model, not the Rust.
//! This test closes part of that gap: it runs every sequence of the model's actions, up to a
//! length, against the real `MotionValue` in a `VirtualDom`, and checks the model's rules that
//! can be observed from outside after every step.
//!
//! | Model action | Here |
//! |---|---|
//! | `Set(x)`, `Stop`, `Animate(t)` | the methods, called from inside the runtime |
//! | `Wake` | one poll of the spawned tasks |
//! | `Unmount` | the owning component goes away |
//!
//! Off the web, without a Tokio runtime, `run()` has no frame timer and finishes an animation
//! on its first turn; so a `Wake` here is a whole animation, and the interleavings that matter
//! (calls between `animate` spawning the loop and the loop's first turn, a loop finding the
//! slot emptied by `set` or `stop`, an unmount while a loop is pending) are all reached.
//!
//! Rules checked: `AnimatingIsTrue` (`is_animating()` matches whether an animation is running),
//! `RestsAtGoal` (once nothing is pending, the value is where the last call asked),
//! `Settles` (every animation finishes), `NoFrozenAnimation` (an animation started after the
//! loop was left in any state still finishes), and `NoWriteAfterUnmount` (no panic when the
//! loop wakes after its component is gone).

use dioxus::prelude::*;
use motion::{use_motion_value, MotionValue, SpringPreset};
use std::cell::{Cell, RefCell};
use std::future::Future;
use std::pin::pin;
use std::task::{Context, Poll, Waker};

thread_local! {
    static VALUE: Cell<Option<MotionValue>> = const { Cell::new(None) };
    static SHOWN: RefCell<Option<Signal<bool>>> = const { RefCell::new(None) };
}

fn app() -> Element {
    let shown = use_signal(|| true);
    SHOWN.with(|s| *s.borrow_mut() = Some(shown));
    rsx! {
        if shown() {
            Holder {}
        }
    }
}

#[component]
fn Holder() -> Element {
    let v = use_motion_value(0.0);
    VALUE.with(|c| c.set(Some(v)));
    rsx! { "{v.peek()}" }
}

#[derive(Clone, Copy, Debug)]
enum Step {
    Set(f64),
    Stop,
    Animate(f64),
    Wake,
    Unmount,
}

const STEPS: [Step; 7] = [
    Step::Set(1.0),
    Step::Set(2.0),
    Step::Stop,
    Step::Animate(1.0),
    Step::Animate(2.0),
    Step::Wake,
    Step::Unmount,
];

/// Run the tasks that are ready, without blocking.
fn wake(dom: &mut VirtualDom) {
    let mut cx = Context::from_waker(Waker::noop());
    for _ in 0..8 {
        let ready = {
            let work = pin!(dom.wait_for_work());
            matches!(work.poll(&mut cx), Poll::Ready(()))
        };
        dom.render_immediate(&mut dioxus::dioxus_core::NoOpMutations);
        if !ready {
            break;
        }
    }
}

/// What the model says the value rests at: the last `set` or `animate` target, or, after a
/// `stop`, wherever it was.
struct Model {
    goal: f64,
    mounted: bool,
}

fn replay(seq: &[Step]) {
    let mut dom = VirtualDom::new(app);
    dom.rebuild_in_place();
    let v = VALUE.with(Cell::get).expect("the holder rendered");
    let mut model = Model {
        goal: 0.0,
        mounted: true,
    };
    let fail = |why: &str| panic!("{why} after {seq:?}");
    for &step in seq {
        match step {
            Step::Set(x) if model.mounted => {
                dom.in_runtime(|| v.set(x));
                model.goal = x;
            }
            Step::Stop if model.mounted => {
                dom.in_runtime(|| v.stop());
                // Without a frame timer an animation never stops between two places: it is
                // still where it started, or finished.
                model.goal = dom.in_runtime(|| v.peek());
            }
            Step::Animate(t) if model.mounted => {
                dom.in_runtime(|| v.spring_to(t, SpringPreset::Snappy));
                model.goal = t;
            }
            Step::Unmount if model.mounted => {
                SHOWN.with(|s| s.borrow().expect("app rendered").set(false));
                dom.render_immediate(&mut dioxus::dioxus_core::NoOpMutations);
                model.mounted = false;
            }
            Step::Wake => wake(&mut dom),
            _ => {}
        }
        if model.mounted {
            // AnimatingIsTrue.
            let (flag, moving) = dom.in_runtime(|| (v.is_animating(), v.is_moving()));
            if flag != moving {
                fail(&format!(
                    "is_animating() = {flag} but an animation running = {moving}"
                ));
            }
        }
    }
    // Settles, RestsAtGoal, NoWriteAfterUnmount: let everything pending run.
    wake(&mut dom);
    if model.mounted {
        let (x, moving) = dom.in_runtime(|| (v.peek(), v.is_moving()));
        if moving {
            fail("an animation never finished");
        }
        if x != model.goal {
            fail(&format!("rests at {x}, not at {}", model.goal));
        }
        // NoFrozenAnimation: whatever state the loop flag was left in, a new animation runs.
        dom.in_runtime(|| v.spring_to(3.0, SpringPreset::Snappy));
        wake(&mut dom);
        let (x, moving) = dom.in_runtime(|| (v.peek(), v.is_moving()));
        if moving || x != 3.0 {
            fail(&format!(
                "a later animation froze at {x} (moving: {moving})"
            ));
        }
    }
}

#[test]
fn every_sequence_of_up_to_six_steps_keeps_the_models_rules() {
    let mut seq = Vec::new();
    let mut count = 0usize;
    fn walk(seq: &mut Vec<Step>, depth: usize, count: &mut usize) {
        if std::env::var_os("REPLAY_DEBUG").is_some() {
            eprintln!("{seq:?}");
        }
        replay(seq);
        *count += 1;
        if depth == 0 {
            return;
        }
        for step in STEPS {
            seq.push(step);
            walk(seq, depth - 1, count);
            seq.pop();
        }
    }
    walk(&mut seq, 6, &mut count);
    // 1 + 7 + 7^2 + ... + 7^6 sequences.
    assert_eq!(count, 137_257);
}
