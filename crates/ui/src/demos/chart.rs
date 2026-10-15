use crate::components::button::{Button, ButtonSize, ButtonVariant};
use crate::components::chart::*;
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

fn months() -> Vec<String> {
    ["Jan", "Feb", "Mar", "Apr", "May", "Jun"]
        .map(String::from)
        .to_vec()
}

fn devices() -> Vec<ChartSeries> {
    vec![
        ChartSeries::new("Desktop", [186.0, 305.0, 237.0, 73.0, 209.0, 214.0]),
        ChartSeries::new("Mobile", [80.0, 200.0, 120.0, 190.0, 130.0, 140.0]),
    ]
}

fn browsers() -> Vec<ChartDatum> {
    [
        ("Chrome", 275.0),
        ("Safari", 200.0),
        ("Firefox", 187.0),
        ("Edge", 173.0),
        ("Other", 90.0),
    ]
    .into_iter()
    .map(|(label, value)| ChartDatum::new(label, value))
    .collect()
}

#[component]
pub fn Demo() -> Element {
    let mut goal = use_signal(|| 64.0_f32);
    rsx! {
        div { style: "display: grid; gap: 2rem; grid-template-columns: repeat(auto-fit, minmax(16rem, 1fr));",
            BarChart { data: monthly(), label: "Visitors per month, January to June" }
            LineChart { data: monthly(), label: "Visitors per month, January to June (line)" }
            AreaChart {
                categories: months(),
                series: devices(),
                stacked: true,
                table: true,
                label: "Visitors by device, January to June, stacked",
            }
            BarChart { categories: months(), series: devices(), label: "Visitors by device, January to June" }
            LineChart { categories: months(), series: devices(), label: "Visitors by device, January to June (lines)" }
            PieChart {
                data: browsers(),
                donut: true,
                center: "935",
                center_caption: "Visitors",
                table: true,
                label: "Visitors by browser",
            }
            PieChart { data: browsers(), label: "Visitors by browser (pie)" }
            div { style: "display: grid; gap: 0.75rem; justify-items: center;",
                RadialChart { value: goal(), label: "Monthly goal reached", caption: "of goal" }
                Button {
                    variant: ButtonVariant::Outline,
                    size: ButtonSize::Sm,
                    onclick: move |_| goal.set(if goal() >= 100.0 { 12.0 } else { (goal() + 22.0).min(100.0) }),
                    "Add progress"
                }
            }
        }
    }
}
