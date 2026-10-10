use super::component::*;
use dioxus::html::FileData;
use dioxus::prelude::*;

#[component]
pub fn Demo() -> Element {
    let mut count = use_signal(|| 0_usize);
    rsx! {
        div { style: "display: grid; gap: 0.75rem; max-width: 28rem;",
            Dropzone {
                accept: "image/*,.pdf",
                multiple: true,
                max_size: 5 * 1024 * 1024,
                on_files: move |files: Vec<FileData>| count.set(count() + files.len()),
            }
            p { style: "margin: 0; font-size: 0.875rem;", role: "status", "Files received: {count()}" }
        }
    }
}
