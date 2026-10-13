//! Generated sample data. Deterministic, so screenshots and tests are stable.

use time::{Date, Month};

pub const PLANS: [&str; 3] = ["Free", "Pro", "Enterprise"];
pub const STATUSES: [&str; 3] = ["Active", "Trial", "Churned"];
pub const ORDER_STATUSES: [&str; 3] = ["Paid", "Pending", "Refunded"];

const FIRST: [&str; 12] = [
    "Ada", "Grace", "Linus", "Margaret", "Alan", "Barbara", "Dennis", "Frances", "Ken", "Radia",
    "Tim", "Hedy",
];
const LAST: [&str; 9] = [
    "Lovelace", "Hopper", "Torvalds", "Hamilton", "Turing", "Liskov", "Ritchie", "Allen", "Lamarr",
];

#[derive(Clone, PartialEq, Debug)]
pub struct Customer {
    pub id: usize,
    pub name: String,
    pub email: String,
    pub plan: String,
    pub status: String,
    /// Monthly revenue in whole dollars.
    pub mrr: u32,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Order {
    pub id: usize,
    pub customer: String,
    pub amount: u32,
    pub status: String,
    pub date: Date,
}

pub fn customers(n: usize) -> Vec<Customer> {
    (0..n)
        .map(|i| {
            let first = FIRST[i % FIRST.len()];
            let last = LAST[(i * 5) % LAST.len()];
            let plan = PLANS[(i * 7) % 3];
            Customer {
                id: i + 1,
                name: format!("{first} {last}"),
                email: format!(
                    "{}.{}{}@example.com",
                    first.to_lowercase(),
                    last.to_lowercase(),
                    i + 1
                ),
                plan: plan.into(),
                status: STATUSES[(i * 3 + i / 4) % 3].into(),
                mrr: match plan {
                    "Free" => 0,
                    "Pro" => 29 + (i as u32 * 13) % 70,
                    _ => 400 + (i as u32 * 97) % 900,
                },
            }
        })
        .collect()
}

pub fn orders(customers: &[Customer], n: usize) -> Vec<Order> {
    let start = Date::from_calendar_date(2026, Month::January, 1).unwrap_or(Date::MIN);
    (0..n)
        .map(|i| Order {
            id: 1000 + i,
            customer: customers[(i * 11) % customers.len()].name.clone(),
            amount: 20 + (i as u32 * 53) % 480,
            status: ORDER_STATUSES[(i / 3 + i) % 3].into(),
            date: start + time::Duration::days(((i * 9) % 280) as i64),
        })
        .collect()
}

/// Revenue per month, Jan to Sep 2026, in thousands of dollars.
pub const REVENUE: [(&str, f32); 9] = [
    ("Jan", 18.2),
    ("Feb", 19.6),
    ("Mar", 21.1),
    ("Apr", 20.4),
    ("May", 23.8),
    ("Jun", 26.0),
    ("Jul", 27.3),
    ("Aug", 29.9),
    ("Sep", 32.4),
];

pub fn money(n: u32) -> String {
    let s = n.to_string();
    let mut out = String::new();
    for (i, c) in s.chars().enumerate() {
        if i > 0 && (s.len() - i).is_multiple_of(3) {
            out.push(',');
        }
        out.push(c);
    }
    format!("${out}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn money_groups_thousands() {
        assert_eq!(money(0), "$0");
        assert_eq!(money(999), "$999");
        assert_eq!(money(1000), "$1,000");
        assert_eq!(money(1234567), "$1,234,567");
    }

    #[test]
    fn data_is_deterministic() {
        let c = customers(40);
        assert_eq!(c, customers(40));
        assert!(c.iter().any(|c| c.status == "Churned"));
        assert_eq!(orders(&c, 50).len(), 50);
    }
}
