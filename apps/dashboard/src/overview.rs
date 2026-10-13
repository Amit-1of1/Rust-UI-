use crate::data::{self, money, PLANS, REVENUE};
use crate::{use_store, Page};
use dioxus::prelude::*;
use ui::prelude::*;

#[component]
pub fn Overview() -> Element {
    let mut store = use_store();
    let customers = store.customers.read();
    let active = customers.iter().filter(|c| c.status == "Active").count();
    let trial = customers.iter().filter(|c| c.status == "Trial").count();
    let churned = customers.iter().filter(|c| c.status == "Churned").count();
    let mrr: u32 = customers
        .iter()
        .filter(|c| c.status != "Churned")
        .map(|c| c.mrr)
        .sum();
    let churn_pct = if customers.is_empty() {
        0.0
    } else {
        (churned as f64 / customers.len() as f64 * 100.0).round()
    };

    let revenue: Vec<ChartDatum> = REVENUE
        .iter()
        .map(|(m, v)| ChartDatum::new(*m, *v))
        .collect();
    let by_plan: Vec<ChartDatum> = PLANS
        .iter()
        .map(|p| ChartDatum::new(*p, customers.iter().filter(|c| c.plan == *p).count() as f32))
        .collect();
    let recent = data::orders(&customers, 5);

    rsx! {
        section { class: "db-kpis", "aria-label": "Key figures",
            Kpi { label: "Monthly revenue", value: money(mrr), note: "+8.4% on last month" }
            Kpi { label: "Active customers", value: active.to_string(), note: "{trial} on trial" }
            Kpi { label: "Churn", value: "{churn_pct}%", note: "{churned} customers" }
            Kpi { label: "Open orders", value: "12", note: "3 need review" }
        }
        div { class: "db-grid-2",
            Card {
                CardHeader {
                    CardTitle { "Revenue" }
                    CardDescription { "Thousands of dollars per month, 2026" }
                }
                CardContent {
                    LineChart { data: revenue, label: "Revenue per month, thousands of dollars" }
                }
            }
            Card {
                CardHeader {
                    CardTitle { "Customers by plan" }
                    CardDescription { "All customers, including churned" }
                }
                CardContent {
                    BarChart { data: by_plan, label: "Customers by plan" }
                }
            }
        }
        Card {
            CardHeader {
                CardTitle { "Recent orders" }
                CardAction {
                    Button {
                        variant: ButtonVariant::Outline,
                        size: ButtonSize::Sm,
                        onclick: move |_| store.page.set(Page::Orders),
                        "View all"
                    }
                }
            }
            CardContent {
                Table {
                    TableCaption { "The five most recent orders" }
                    TableHeader {
                        TableRow {
                            TableHead { "Order" }
                            TableHead { "Customer" }
                            TableHead { "Status" }
                            TableHead { style: "text-align: right;", "Amount" }
                        }
                    }
                    TableBody {
                        for o in recent {
                            TableRow { key: "{o.id}",
                                TableCell { "#{o.id}" }
                                TableCell { "{o.customer}" }
                                TableCell {
                                    Badge { variant: order_badge(&o.status), "{o.status}" }
                                }
                                TableCell { style: "text-align: right;", {money(o.amount)} }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn order_badge(status: &str) -> BadgeVariant {
    match status {
        "Paid" => BadgeVariant::Primary,
        "Refunded" => BadgeVariant::Destructive,
        _ => BadgeVariant::Secondary,
    }
}

#[component]
fn Kpi(label: String, value: String, note: String) -> Element {
    rsx! {
        Card { class: "db-kpi",
            CardHeader {
                CardDescription { "{label}" }
                CardTitle { class: "db-kpi-value", "{value}" }
            }
            CardContent {
                p { class: "db-muted", "{note}" }
            }
        }
    }
}
