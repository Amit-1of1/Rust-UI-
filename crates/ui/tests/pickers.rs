use ui::components::dropzone::{accepts, format_size};
use ui::components::multi_select::{filter_options, summary, MultiOption};

#[test]
fn accept_lists_match_extensions_wildcards_and_exact_types() {
    assert!(accepts("", "anything.bin", ""));
    assert!(accepts(".PNG, .jpg", "Photo.png", "image/png"));
    assert!(!accepts(".png", "photo.jpeg", "image/jpeg"));
    assert!(accepts("image/*", "x", "image/webp"));
    assert!(!accepts("image/*", "x", "application/pdf"));
    assert!(accepts("application/pdf", "doc", "application/pdf"));
    assert!(
        !accepts("application/pdf", "doc.pdf", ""),
        "type is checked when only a mime is listed"
    );
}

#[test]
fn sizes_read_naturally() {
    assert_eq!(format_size(0), "0 B");
    assert_eq!(format_size(1023), "1023 B");
    assert_eq!(format_size(1536), "1.5 KB");
    assert_eq!(format_size(5 * 1024 * 1024), "5.0 MB");
}

#[test]
fn summary_shows_up_to_two_labels_then_a_count() {
    assert_eq!(summary(&[], "Pick"), "Pick");
    assert_eq!(summary(&["A"], "Pick"), "A");
    assert_eq!(summary(&["A", "B"], "Pick"), "A, B");
    assert_eq!(summary(&["A", "B", "C", "D"], "Pick"), "A, B +2");
}

#[test]
fn filter_needs_every_word_in_the_label() {
    let options = vec![
        MultiOption::new("1", "Cloud Platform"),
        MultiOption::new("2", "Design"),
    ];
    assert_eq!(filter_options(&options, "").len(), 2);
    assert_eq!(filter_options(&options, "CLOUD plat").len(), 1);
    assert_eq!(filter_options(&options, "zzz").len(), 0);
}
