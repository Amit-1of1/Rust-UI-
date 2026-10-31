use crate::components::resizable::*;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut layout = use_signal(|| vec![30.0_f32, 70.0]);
    let pane = "padding: 1rem; font-size: 0.875rem;";
    rsx! {
        div { style: "display: grid; gap: 0.5rem; max-width: 36rem;",
            div { style: "height: 14rem; border: 1px solid var(--primary-color-6); border-radius: var(--radius);",
                ResizablePanelGroup { on_layout: move |sizes| layout.set(sizes),
                    ResizablePanel { id: "resizable-folders", default_size: 30.0, min_size: 15.0, max_size: 60.0,
                        div { style: pane, "Folders" }
                    }
                    ResizableHandle { aria_label: "Resize folders", with_handle: true }
                    ResizablePanel { default_size: 70.0, min_size: 30.0,
                        ResizablePanelGroup { direction: ResizableDirection::Vertical,
                            ResizablePanel { id: "resizable-messages", default_size: 60.0, min_size: 20.0,
                                div { style: pane, "Messages" }
                            }
                            ResizableHandle { aria_label: "Resize preview" }
                            ResizablePanel { default_size: 40.0, min_size: 20.0,
                                div { style: pane, "Preview" }
                            }
                        }
                    }
                }
            }
            p { style: "margin: 0; font-size: 0.875rem;",
                "Folders take {layout()[0].round()}%"
            }
        }
    }
}
