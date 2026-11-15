//! `format_size`: the number shown is always under 1024 of its unit (except gigabytes) and
//! has at most one decimal.

use proptest::prelude::*;
use ui::components::dropzone::format_size;

fn check(bytes: u64) {
    let s = format_size(bytes);
    let (num, unit) = s.split_once(' ').unwrap();
    let v: f64 = num.parse().unwrap();
    if unit != "GB" {
        assert!(v < 1024.0, "{bytes} bytes shows as {s}");
    }
    assert!(["B", "KB", "MB", "GB"].contains(&unit), "{s}");
}

#[test]
fn the_shown_number_stays_under_1024_at_every_unit_edge() {
    assert_eq!(format_size(1_048_535), "1.0 MB"); // 1023.96 KB used to show "1024.0 KB"
    for unit in [1u64, 1 << 10, 1 << 20, 1 << 30] {
        for k in 1000..1100u64 {
            for d in 0..64 {
                check((k * unit).saturating_add(d));
                check((k * unit).saturating_sub(d));
            }
        }
    }
}

proptest! {
    #[test]
    fn any_size(bytes in any::<u64>()) {
        check(bytes);
    }
}
