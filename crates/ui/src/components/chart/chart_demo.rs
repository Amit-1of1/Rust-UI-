use super::component::*;
use dioxus::prelude::*;

fn monthly() -> Vec<ChartDatum> {
    [
        ("Jan", 186.0),
        ("Feb", 305.0),
        ("Mar", 237.0),
        ("Apr", 73.0),
        ("May", 209.0),
        ("Jun", 214.0),
    ]
    .into_iter()
    .map(|(label, value)| ChartDatum::new(label, value))
    .collect()
}

#[component]
pub fn Demo() -> Element {
    rsx! {
        div { style: "display: grid; gap: 2rem; grid-template-columns: repeat(auto-fit, minmax(16rem, 1fr));",
            BarChart { data: monthly(), label: "Visitors per month, January to June" }
            LineChart { data: monthly(), label: "Visitors per month, January to June (line)" }
        }
    }
}
