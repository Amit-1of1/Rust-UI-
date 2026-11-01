use crate::components::image::*;
use dioxus::prelude::*;

/// A landscape drawn inline (no network in demos), at two densities via `srcset`.
const SCENE: &str = concat!(
    "data:image/svg+xml;utf8,",
    "<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 160 90'>",
    "<rect width='160' height='90' fill='%23bfdbfe'/>",
    "<circle cx='122' cy='24' r='10' fill='%23fde68a'/>",
    "<path d='M0 70 40 36l26 22 22-16 72 48H0z' fill='%2394a3b8'/></svg>"
);

#[component]
pub fn Demo() -> Element {
    let srcset = format!("{SCENE} 1x, {SCENE} 2x");
    rsx! {
        div { style: "display: flex; flex-wrap: wrap; gap: 1rem; align-items: flex-start;",
            Image { style: "width: 12rem;", src: SCENE, srcset, alt: "Hills under a pale sun", ratio: 16.0 / 9.0 }
            Image { style: "width: 6rem;", src: SCENE, alt: "Hills, cropped square", ratio: 1.0 }
            Image { style: "width: 9rem;", src: SCENE, alt: "Hills, letterboxed", ratio: 1.0, fit: ImageFit::Contain }
            // Empty source: the fallback, named by `alt`.
            Image { style: "width: 9rem;", src: "", alt: "Profile banner (missing)", ratio: 16.0 / 9.0 }
        }
    }
}
