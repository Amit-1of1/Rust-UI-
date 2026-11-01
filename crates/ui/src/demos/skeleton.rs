use crate::components::skeleton::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { style: "display: flex; flex-direction: column; align-items: center; gap: 2rem;",
            SkeletonInfoDemo {}
            SkeletonCardDemo {}
            SkeletonShapesDemo {}
        }
    }
}

#[component]
fn SkeletonInfoDemo() -> Element {
    rsx! {
        div { style: "display: flex; align-items: center; gap: 1rem;",
            Skeleton { style: "width: 3rem; height: 3rem; border-radius: 50%;" }
            div { style: "display: flex; flex-direction: column; gap: 0.5rem;",
                Skeleton { style: "width: 11.625rem; height: 1rem;" }
                Skeleton { style: "width: 8.5rem; height: 1rem;" }
            }
        }
    }
}

#[component]
fn SkeletonCardDemo() -> Element {
    rsx! {
        div { style: "display: flex; flex-direction: column; gap: 0.75rem;",
            Skeleton { style: "width: 15rem; height: 8rem; border-radius: 0.75rem;" }
            div { style: "display: flex; flex-direction: column; gap: 0.5rem;",
                Skeleton { style: "width: 15.625rem; height: 1rem;" }
                Skeleton { style: "width: 12.5rem; height: 1rem;" }
            }
        }
    }
}

/// The shapes: a circle for an avatar and text lines that follow the font size.
#[component]
fn SkeletonShapesDemo() -> Element {
    rsx! {
        div { style: "display: flex; align-items: flex-start; gap: 1rem; width: 16rem;", aria_busy: "true",
            Skeleton { shape: SkeletonShape::Circle, style: "width: 2.5rem;" }
            div { style: "flex: 1; font-size: 0.875rem;",
                Skeleton { shape: SkeletonShape::Text, lines: 3 }
            }
        }
    }
}
