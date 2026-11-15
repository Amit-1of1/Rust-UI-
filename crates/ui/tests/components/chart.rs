//! `nice_max`, the axis limit: always at or above the data, always 1, 2 or 5 times a power of
//! ten (or the data itself when float rounding leaves no nice value above it), at most 2.5 times
//! the data.

use ui::components::chart::nice_max;

fn check(x: f32) {
    let m = nice_max(x);
    if !(x.is_finite() && x > 0.0) {
        assert_eq!(m, 1.0, "nice_max({x:e})");
        return;
    }
    assert!(m >= x, "nice_max({x:e}) = {m:e} is below the data");
    assert!(
        f64::from(m) <= 2.5 * f64::from(x) * (1.0 + 1e-6) || m == f32::MAX,
        "nice_max({x:e}) = {m:e} is more than 2.5 times the data"
    );
}

/// Every power of ten times 1, 2 and 5 that f32 can hold, and the 64 floats either side of it:
/// where `10f32.powf` rounds low (0.05 used to give 0.049999997).
#[test]
fn nice_max_is_never_below_the_data_near_nice_values() {
    let mut n = 0;
    for k in -38..=38 {
        for base in [1.0f64, 2.0, 5.0] {
            let x = (base * 10f64.powi(k)) as f32;
            if !x.is_normal() {
                continue;
            }
            for d in -64i32..=64 {
                let bits = (x.to_bits() as i64 + i64::from(d)) as u32;
                check(f32::from_bits(bits));
                n += 1;
            }
        }
    }
    assert!(n > 25_000);
}

/// One float in every 4,099 bit patterns: about a million values across the whole range,
/// including negatives, subnormals, infinities and NaNs.
#[test]
fn nice_max_holds_across_the_f32_range() {
    let mut bits: u64 = 0;
    while bits <= u64::from(u32::MAX) {
        check(f32::from_bits(bits as u32));
        bits += 4_099;
    }
}

/// Every one of the 2^32 floats (about a minute in release):
/// `cargo test --release -p atoll-ui --test components chart::nice_max_every_f32 -- --ignored`
#[test]
#[ignore]
fn nice_max_every_f32() {
    for bits in 0..=u32::MAX {
        check(f32::from_bits(bits));
    }
}
