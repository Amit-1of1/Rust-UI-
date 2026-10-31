use super::render;
use dioxus::prelude::*;
use ui::prelude::*;

#[test]
fn native_select_is_a_real_select_with_its_options_and_attributes() {
    let html = render(|| {
        rsx! {
            NativeSelect { id: "plan", name: "plan", required: true, class: "mine",
                NativeSelectOption { value: "a", "Alpha" }
                NativeSelectGroup { label: "More",
                    NativeSelectOption { value: "b", disabled: true, "Beta" }
                }
            }
        }
    });
    assert!(html.contains("<select"), "{html}");
    assert!(html.contains("id=\"plan\""), "{html}");
    assert!(html.contains("name=\"plan\""), "{html}");
    assert!(html.contains("<option value=\"a\""), "{html}");
    assert!(html.contains("<optgroup label=\"More\""), "{html}");
    assert!(html.contains("mine"), "caller's class kept: {html}");
    assert!(html.contains("dx-native-select"), "kit's class kept: {html}");
    assert!(html.contains("aria-hidden=\"true\""), "chevron hidden: {html}");
}
