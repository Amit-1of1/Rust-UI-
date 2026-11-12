use crate::components::data_table::*;
use crate::components::toggle::Toggle;
use dioxus::prelude::*;

const PEOPLE: [(&str, &str, &str, &str); 14] = [
    ("Ada Lovelace", "Engineering", "London", "$142,000"),
    ("Grace Hopper", "Engineering", "New York", "$151,500"),
    ("Alan Turing", "Research", "Manchester", "$138,250"),
    ("Katherine Johnson", "Research", "Hampton", "$129,900"),
    ("Margaret Hamilton", "Engineering", "Boston", "$147,300"),
    ("Linus Torvalds", "Platform", "Portland", "$160,000"),
    ("Barbara Liskov", "Research", "Cambridge", "$155,750"),
    ("Dennis Ritchie", "Platform", "Murray Hill", "$133,400"),
    ("Hedy Lamarr", "Design", "Vienna", "$98,000"),
    ("Tim Berners-Lee", "Platform", "Geneva", "$149,000"),
    ("Radia Perlman", "Platform", "Redmond", "$144,800"),
    ("Donald Knuth", "Research", "Stanford", "$170,000"),
    ("Frances Allen", "Engineering", "Yorktown", "$140,100"),
    ("Edsger Dijkstra", "Research", "Eindhoven", "$137,600"),
];

#[component]
pub fn Demo() -> Element {
    // An outside filter, shown in the table's toolbar beside its own filter box.
    let mut research_only = use_signal(|| false);
    let rows: Vec<Vec<String>> = PEOPLE
        .iter()
        .filter(|(_, team, _, _)| !research_only() || *team == "Research")
        .map(|(a, b, c, d)| vec![a.to_string(), b.to_string(), c.to_string(), d.to_string()])
        .collect();
    let columns = vec![
        DataColumn::new("Name").sortable().always_visible(),
        DataColumn::new("Team").sortable(),
        DataColumn::new("Office"),
        DataColumn::new("Salary").sortable().numeric(),
    ];
    let mut picked = use_signal(Vec::<usize>::new);
    let orders = use_hook(orders);
    rsx! {
        div { style: "display: grid; gap: 0.75rem;",
            DataTable {
                columns,
                rows,
                caption: "Team directory",
                page_size: 5,
                selectable: true,
                on_select: move |s| picked.set(s),
                column_menu: true,
                resizable_columns: true,
                exportable: true,
                toolbar: rsx! {
                    Toggle {
                        pressed: research_only(),
                        on_pressed_change: move |on| research_only.set(on),
                        "Research only"
                    }
                },
            }
            p { style: "margin: 0; font-size: 0.875rem;", "Selected rows: {picked().len()}" }
            // A long list: only the rows in view are rendered, so 10,000 cost what 20 do.
            DataTable {
                columns: vec![
                    DataColumn::new("Order").sortable(),
                    DataColumn::new("Customer").sortable(),
                    DataColumn::new("Status"),
                    DataColumn::new("Total").sortable().numeric(),
                ],
                rows: orders.clone(),
                caption: "10,000 orders",
                virtual_height: 360.0,
                exportable: true,
            }
        }
    }
}

/// 10,000 made-up orders for the virtualised table, the same on every run.
fn orders() -> Vec<Vec<String>> {
    const CUSTOMERS: [&str; 6] = ["Acme", "Globex", "Initech", "Umbrella", "Hooli", "Vandelay"];
    const STATUS: [&str; 4] = ["Paid", "Pending", "Refunded", "Shipped"];
    (0..10_000_usize)
        .map(|i| {
            vec![
                format!("#{:05}", i + 1),
                CUSTOMERS[(i * 7) % CUSTOMERS.len()].to_string(),
                STATUS[(i * 3) % STATUS.len()].to_string(),
                format!("${}.{:02}", (i * 37) % 5000 + 5, (i * 13) % 100),
            ]
        })
        .collect()
}
