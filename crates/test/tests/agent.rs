//! The MCP server an AI agent drives (`atoll_test::agent`): the same calls an agent makes,
//! answered by a headless screen.

use atoll_test::agent::Agent;
use atoll_test::prelude::*;
use dioxus::prelude::*;
use serde_json::{json, Value};
use ui::prelude::*;

fn signup() -> Element {
    let mut email = use_signal(String::new);
    let mut sent = use_signal(|| false);
    rsx! {
        ThemeStyles { reset: true }
        main { style: "padding: 1.5rem;",
            h1 { "Join the beta" }
            Label { html_for: "email", "Email" }
            Input { id: "email", value: email, oninput: move |e: FormEvent| email.set(e.value()) }
            Button { onclick: move |_| sent.set(true), "Sign up" }
            if sent() {
                p { role: "status", "Thanks, {email}!" }
            }
        }
    }
}

fn call(agent: &mut Agent, id: u64, name: &str, args: Value) -> Value {
    agent
        .handle(&json!({ "jsonrpc": "2.0", "id": id, "method": "tools/call", "params": { "name": name, "arguments": args } }))
        .expect("a reply")["result"]
        .clone()
}

fn text_of(result: &Value) -> String {
    result["content"][0]["text"]
        .as_str()
        .unwrap_or_default()
        .to_string()
}

#[test]
fn an_agent_reads_fills_clicks_and_sees_the_result() {
    let mut agent = Agent::new(Screen::render(signup));

    let init = agent
        .handle(&json!({ "jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {} }))
        .unwrap();
    assert_eq!(init["result"]["serverInfo"]["name"], "atoll-screen");
    assert!(agent
        .handle(&json!({ "jsonrpc": "2.0", "method": "notifications/initialized" }))
        .is_none());
    let listed = agent
        .handle(&json!({ "jsonrpc": "2.0", "id": 2, "method": "tools/list" }))
        .unwrap();
    let names: Vec<&str> = listed["result"]["tools"]
        .as_array()
        .unwrap()
        .iter()
        .map(|t| t["name"].as_str().unwrap())
        .collect();
    assert_eq!(
        names,
        [
            "outline",
            "click",
            "fill",
            "press",
            "type_text",
            "wait",
            "read",
            "screenshot"
        ]
    );

    let outline = text_of(&call(&mut agent, 3, "outline", json!({})));
    assert!(outline.contains("heading \"Join the beta\""), "{outline}");
    assert!(outline.contains("textbox \"Email\""), "{outline}");
    assert!(outline.contains("button \"Sign up\""), "{outline}");

    let filled = call(
        &mut agent,
        4,
        "fill",
        json!({ "label": "Email", "value": "ada@example.com" }),
    );
    assert!(filled.get("isError").is_none(), "{filled}");

    let clicked = text_of(&call(
        &mut agent,
        5,
        "click",
        json!({ "role": "button", "name": "Sign up" }),
    ));
    assert!(
        clicked.contains("status \"Thanks, ada@example.com!\"")
            || clicked.contains("Thanks, ada@example.com!"),
        "{clicked}"
    );

    let shot = call(&mut agent, 6, "screenshot", json!({}));
    assert_eq!(shot["content"][0]["mimeType"], "image/png");
    assert!(
        shot["content"][0]["data"]
            .as_str()
            .unwrap()
            .starts_with("iVBORw0KGgo"),
        "PNG signature in base64"
    );
}

#[test]
fn a_bad_query_is_an_error_for_the_agent_not_a_crash() {
    let mut agent = Agent::new(Screen::render(signup));
    let r = call(
        &mut agent,
        1,
        "click",
        json!({ "role": "button", "name": "Delete account" }),
    );
    assert_eq!(r["isError"], true);
    let msg = text_of(&r);
    assert!(msg.contains("Delete account"), "{msg}");
    // The server is still alive.
    let ok = call(&mut agent, 2, "outline", json!({}));
    assert!(ok.get("isError").is_none());
    let missing = call(&mut agent, 3, "click", json!({}));
    assert_eq!(missing["isError"], true);
}
