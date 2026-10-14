//! Made-up stays and trips. Everything is generated from fixed lists, so every run (and every
//! test) sees the same data; only dates move with today's date.

use time::{Date, Duration};
use ui::prelude::DateRange;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    Cabin,
    Yurt,
    Treehouse,
    Tent,
}

impl Kind {
    pub const ALL: [Kind; 4] = [Kind::Cabin, Kind::Yurt, Kind::Treehouse, Kind::Tent];

    pub fn label(self) -> &'static str {
        match self {
            Kind::Cabin => "Cabin",
            Kind::Yurt => "Yurt",
            Kind::Treehouse => "Treehouse",
            Kind::Tent => "Safari tent",
        }
    }
}

pub const AMENITIES: [&str; 6] = [
    "Wi-Fi",
    "Hot tub",
    "Pets allowed",
    "Wood stove",
    "Kitchen",
    "EV charger",
];

#[derive(Clone, PartialEq, Debug)]
pub struct Stay {
    pub id: usize,
    pub name: String,
    pub region: &'static str,
    pub kind: Kind,
    /// Per night, in dollars.
    pub price: u32,
    /// Rating times ten (47 = 4.7), so the struct stays `Eq`-friendly.
    pub rating: u32,
    pub reviews: u32,
    pub max_guests: u32,
    pub amenities: Vec<&'static str>,
    pub host: &'static str,
    /// Hue for the placeholder "photos" (demos never load remote images).
    pub hue: u32,
}

impl Stay {
    pub fn rating_text(&self) -> String {
        format!("{}.{}", self.rating / 10, self.rating % 10)
    }
}

const NAMES: [&str; 12] = [
    "Fern Hollow",
    "Lookout Ridge",
    "Cedar Bend",
    "Moss Creek",
    "Quiet Pines",
    "Granite Lake",
    "Owl's Rest",
    "Birch Meadow",
    "Foxglove",
    "Stillwater",
    "High Larch",
    "Juniper Flat",
];
const REGIONS: [&str; 4] = ["Cascades", "Lake District", "Black Forest", "Blue Ridge"];
const HOSTS: [&str; 6] = [
    "Maya Okafor",
    "Lars Eriksen",
    "Priya Nair",
    "Tomás Ruiz",
    "Hana Sato",
    "Owen Price",
];

pub fn stays() -> Vec<Stay> {
    (0..24)
        .map(|i| {
            let kind = Kind::ALL[(i * 7 / 3) % 4];
            Stay {
                id: i,
                name: format!("{} {}", NAMES[i % 12], kind.label()),
                region: REGIONS[(i * 5) % 4],
                kind,
                price: 70 + ((i * 37) % 300) as u32,
                rating: 40 + ((i * 3) % 10) as u32,
                reviews: 8 + ((i * 29) % 200) as u32,
                max_guests: 2 + (i % 5) as u32,
                amenities: AMENITIES
                    .iter()
                    .enumerate()
                    .filter(|(a, _)| !(i + a * 3).is_multiple_of(4))
                    .map(|(_, name)| *name)
                    .collect(),
                host: HOSTS[i % 6],
                hue: ((i * 47) % 360) as u32,
            }
        })
        .collect()
}

pub struct Review {
    pub author: &'static str,
    pub when: &'static str,
    pub text: &'static str,
}

pub const REVIEWS: [Review; 3] = [
    Review {
        author: "Ines W.",
        when: "August",
        text: "Spotless and quiet. The stove kept us warm through a cold night.",
    },
    Review {
        author: "Dev K.",
        when: "July",
        text: "Great base for the trails. The road in is gravel, so drive slowly.",
    },
    Review {
        author: "Sam R.",
        when: "June",
        text: "Exactly like the photos. We'd book it again.",
    },
];

pub const CLEANING_FEE: u32 = 45;
/// Service fee, percent of the nightly total.
pub const SERVICE_PERCENT: u32 = 8;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Rate {
    /// Free cancellation until 48 hours before, 10% more.
    Flexible,
    NonRefundable,
}

impl Rate {
    pub fn id(self) -> &'static str {
        match self {
            Rate::Flexible => "flexible",
            Rate::NonRefundable => "non-refundable",
        }
    }

    pub fn from_id(id: &str) -> Rate {
        if id == "non-refundable" {
            Rate::NonRefundable
        } else {
            Rate::Flexible
        }
    }
}

/// Price breakdown for `nights` at `price`.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Quote {
    pub nights: u32,
    pub lodging: u32,
    pub cleaning: u32,
    pub service: u32,
    pub total: u32,
}

pub fn quote(price: u32, nights: u32, rate: Rate) -> Quote {
    let nightly = match rate {
        Rate::Flexible => price + price / 10,
        Rate::NonRefundable => price,
    };
    let lodging = nightly * nights;
    let service = lodging * SERVICE_PERCENT / 100;
    Quote {
        nights,
        lodging,
        cleaning: CLEANING_FEE,
        service,
        total: lodging + CLEANING_FEE + service,
    }
}

/// Two stays overlap when one starts before the other ends (check-out day is free again).
pub fn overlaps(a: &DateRange, b: &DateRange) -> bool {
    a.start() < b.end() && b.start() < a.end()
}

pub fn nights(range: &DateRange) -> u32 {
    (range.end() - range.start()).whole_days().max(0) as u32
}

pub fn money(dollars: u32) -> String {
    let s = dollars.to_string();
    let mut out = String::new();
    for (i, c) in s.chars().enumerate() {
        if i > 0 && (s.len() - i).is_multiple_of(3) {
            out.push(',');
        }
        out.push(c);
    }
    format!("${out}")
}

pub fn today() -> Date {
    time::OffsetDateTime::now_utc().date()
}

/// "Oct 3 – Oct 7, 2026"
pub fn date_range_text(range: &DateRange) -> String {
    let short = |d: Date| format!("{} {}", &d.month().to_string()[..3], d.day());
    format!(
        "{} – {}, {}",
        short(range.start()),
        short(range.end()),
        range.end().year()
    )
}

/// Nights already booked at a stay, so the picker greys them out.
pub fn booked(stay: usize) -> Vec<DateRange> {
    let start = today() + Duration::days(3 + (stay % 5) as i64);
    vec![DateRange::new(start, start + Duration::days(2))]
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TripStatus {
    Confirmed,
    Cancelled,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Trip {
    pub id: u32,
    pub stay: usize,
    pub range: DateRange,
    pub guests: u32,
    pub rate: Rate,
    pub guest_name: String,
    pub total: u32,
    pub status: TripStatus,
}

/// Two trips so "Upcoming" and "Past" both have something before the user books.
pub fn seed_trips(stays: &[Stay]) -> Vec<Trip> {
    let t = today();
    let trip = |id: u32, stay: usize, from: i64, n: i64| {
        let range = DateRange::new(t + Duration::days(from), t + Duration::days(from + n));
        Trip {
            id,
            stay,
            range,
            guests: 2,
            rate: Rate::Flexible,
            guest_name: "Alex Lee".into(),
            total: quote(stays[stay].price, n as u32, Rate::Flexible).total,
            status: TripStatus::Confirmed,
        }
    };
    vec![trip(1001, 4, 30, 3), trip(1000, 9, -60, 2)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn money_groups_thousands() {
        assert_eq!(money(5), "$5");
        assert_eq!(money(1234), "$1,234");
        assert_eq!(money(1234567), "$1,234,567");
    }

    #[test]
    fn quote_adds_fees() {
        let q = quote(100, 3, Rate::NonRefundable);
        assert_eq!(q.lodging, 300);
        assert_eq!(q.service, 24);
        assert_eq!(q.total, 300 + 24 + CLEANING_FEE);
        assert_eq!(quote(100, 1, Rate::Flexible).lodging, 110);
    }

    #[test]
    fn stays_are_varied() {
        let all = stays();
        assert_eq!(all.len(), 24);
        for kind in Kind::ALL {
            assert!(all.iter().any(|s| s.kind == kind), "{kind:?} missing");
        }
        assert!(all.iter().all(|s| !s.amenities.is_empty()));
    }
}
