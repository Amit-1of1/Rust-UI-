use crate::components::data_table::*;
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
    let rows: Vec<Vec<String>> = PEOPLE
        .iter()
        .map(|(a, b, c, d)| vec![a.to_string(), b.to_string(), c.to_string(), d.to_string()])
        .collect();
    let columns = vec![
        DataColumn::new("Name").sortable(),
        DataColumn::new("Team").sortable(),
        DataColumn::new("Office"),
        DataColumn::new("Salary").sortable().numeric(),
    ];
    let mut picked = use_signal(Vec::<usize>::new);
    rsx! {
        div { style: "display: grid; gap: 0.75rem;",
            DataTable {
                columns,
                rows,
                caption: "Team directory",
                page_size: 5,
                selectable: true,
                on_select: move |s| picked.set(s),
            }
            p { style: "margin: 0; font-size: 0.875rem;", "Selected rows: {picked().len()}" }
        }
    }
}
