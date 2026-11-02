use crate::components::combobox::*;
use dioxus::prelude::*;
use dioxus_sdk_time::sleep;
use std::time::Duration;

const COUNTRIES: &[&str] = &[
    "Argentina", "Australia", "Austria", "Belgium", "Brazil", "Canada", "Chile", "China",
    "Colombia", "Croatia", "Czechia", "Denmark", "Egypt", "Estonia", "Finland", "France",
    "Germany", "Ghana", "Greece", "Hungary", "Iceland", "India", "Indonesia", "Ireland",
    "Israel", "Italy", "Japan", "Kenya", "Latvia", "Lithuania", "Malaysia", "Mexico",
    "Morocco", "Netherlands", "New Zealand", "Nigeria", "Norway", "Peru", "Philippines",
    "Poland", "Portugal", "Romania", "Senegal", "Singapore", "Slovenia", "South Africa",
    "South Korea", "Spain", "Sweden", "Switzerland", "Thailand", "Tunisia", "Turkey",
    "Ukraine", "United Kingdom", "United States", "Uruguay", "Vietnam",
];

/// Stands in for a server: 400 ms later, one page of six countries that contain `query`.
/// Demos never touch the network.
async fn search_countries(query: String, page: usize) -> Result<AsyncPage<String>, String> {
    sleep(Duration::from_millis(400)).await;
    let q = query.trim().to_lowercase();
    let found: Vec<String> = COUNTRIES
        .iter()
        .filter(|c| c.to_lowercase().contains(&q))
        .map(|c| c.to_string())
        .collect();
    let items: Vec<String> = found.iter().skip(page * 6).take(6).cloned().collect();
    Ok(AsyncPage {
        has_more: found.len() > (page + 1) * 6,
        items,
    })
}

#[component]
pub fn Demo() -> Element {
    let countries = use_async_options(search_countries);

    rsx! {
        AsyncCombobox::<String> {
            options: countries,
            label: |country: String| country,
            placeholder: "Search countries...",
            aria_label: "Country",
            list_aria_label: "Countries",
        }
    }
}
