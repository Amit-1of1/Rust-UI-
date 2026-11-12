use super::render;
use dioxus::prelude::*;
use ui::prelude::*;

#[test]
fn image_renders_the_picture_with_its_ratio_and_srcset() {
    let html = render(|| {
        rsx! { Image { src: "/a.png", srcset: "/a.png 1x, /a@2x.png 2x", alt: "A lake", ratio: 2.0 } }
    });
    assert!(html.contains("aspect-ratio: 2"), "{html}");
    assert!(html.contains("srcset=\"/a.png 1x, /a@2x.png 2x\""), "{html}");
    assert!(html.contains("alt=\"A lake\""), "{html}");
    assert!(html.contains("loading=\"lazy\""), "{html}");
}

#[test]
fn an_empty_source_shows_a_named_fallback() {
    let html = render(|| rsx! { Image { src: "", alt: "Banner" } });
    assert!(!html.contains("<img"), "{html}");
    assert!(html.contains("role=\"img\""), "{html}");
    assert!(html.contains("aria-label=\"Banner\""), "{html}");
    let html = render(|| rsx! { Image { src: "", alt: "" } });
    assert!(html.contains("aria-hidden=\"true\""), "a decorative fallback is hidden: {html}");
}
