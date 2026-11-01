//! `atoll mcp` spoken to over stdio, the way an MCP client does: start the binary, write
//! one JSON-RPC message per line, read one reply per line.

use serde_json::{json, Value};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

struct Client {
    child: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
    next_id: u64,
}

impl Client {
    /// A server reading this repo and writing into `project`, already initialized.
    fn start(project: &Path) -> Self {
        let mut child = Command::new(env!("CARGO_BIN_EXE_atoll"))
            .arg("--source")
            .arg(repo_root())
            .arg("mcp")
            .current_dir(project)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("start atoll mcp");
        let stdin = child.stdin.take().unwrap();
        let stdout = BufReader::new(child.stdout.take().unwrap());
        let mut client = Client {
            child,
            stdin,
            stdout,
            next_id: 0,
        };
        let init = client.request(
            "initialize",
            json!({
                "protocolVersion": "2025-06-18",
                "capabilities": {},
                "clientInfo": { "name": "test", "version": "0" }
            }),
        );
        assert_eq!(init["result"]["protocolVersion"], "2025-06-18");
        assert_eq!(init["result"]["serverInfo"]["name"], "atoll");
        assert!(init["result"]["capabilities"]["tools"].is_object());
        client.send(&json!({ "jsonrpc": "2.0", "method": "notifications/initialized" }));
        client
    }

    fn send(&mut self, message: &Value) {
        writeln!(self.stdin, "{message}").unwrap();
        self.stdin.flush().unwrap();
    }

    fn read(&mut self) -> Value {
        let mut line = String::new();
        self.stdout.read_line(&mut line).unwrap();
        assert!(line.ends_with('\n'), "one message per line: {line:?}");
        serde_json::from_str(&line).unwrap_or_else(|e| panic!("not JSON ({e}): {line}"))
    }

    fn request(&mut self, method: &str, params: Value) -> Value {
        self.next_id += 1;
        let id = self.next_id;
        self.send(&json!({ "jsonrpc": "2.0", "id": id, "method": method, "params": params }));
        let reply = self.read();
        assert_eq!(reply["jsonrpc"], "2.0");
        assert_eq!(reply["id"], id, "{reply}");
        reply
    }

    /// Call a tool; returns (text, is_error).
    fn call(&mut self, tool: &str, arguments: Value) -> (String, bool) {
        let reply = self.request(
            "tools/call",
            json!({ "name": tool, "arguments": arguments }),
        );
        let result = &reply["result"];
        let text = result["content"][0]["text"]
            .as_str()
            .unwrap_or_else(|| panic!("no text content: {reply}"))
            .to_string();
        (text, result["isError"] == true)
    }

    fn call_json(&mut self, tool: &str, arguments: Value) -> Vec<Value> {
        let (text, is_error) = self.call(tool, arguments);
        assert!(!is_error, "{text}");
        serde_json::from_str(&text).unwrap()
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

fn names(rows: &[Value]) -> Vec<&str> {
    rows.iter().map(|r| r["name"].as_str().unwrap()).collect()
}

#[test]
fn lists_the_tools_with_schemas() {
    let tmp = tempfile::tempdir().unwrap();
    let mut c = Client::start(tmp.path());
    let reply = c.request("tools/list", json!({}));
    let tools = reply["result"]["tools"].as_array().unwrap();
    let listed: Vec<&str> = tools.iter().map(|t| t["name"].as_str().unwrap()).collect();
    assert_eq!(
        listed,
        [
            "list_components",
            "search_components",
            "get_component",
            "add_component",
            "init_theme",
            "explain_errors",
            "get_contract",
            "generate_theme",
            "list_templates"
        ]
    );
    for t in tools {
        assert_eq!(t["inputSchema"]["type"], "object", "{t}");
        assert!(t["description"].as_str().is_some_and(|d| !d.is_empty()));
    }
    assert_eq!(c.request("ping", json!({}))["result"], json!({}));
}

#[test]
fn list_matches_the_registry() {
    let tmp = tempfile::tempdir().unwrap();
    let mut c = Client::start(tmp.path());
    let reg = atoll_cli::load(&repo_root()).unwrap();
    let all = c.call_json("list_components", json!({}));
    assert_eq!(all.len(), reg.items.len());
    let dialog = all.iter().find(|r| r["name"] == "dialog").unwrap();
    assert_eq!(dialog["kind"], "component");
    assert_eq!(dialog["title"], "Dialog");
    let blocks = c.call_json("list_components", json!({ "kind": "block" }));
    assert!(!blocks.is_empty());
    assert!(blocks.iter().all(|r| r["kind"] == "block"));
    assert!(names(&blocks).contains(&"login_01"));
    let (text, is_error) = c.call("list_components", json!({ "kind": "widgets" }));
    assert!(is_error, "{text}");
}

#[test]
fn search_ranks_names_first() {
    let tmp = tempfile::tempdir().unwrap();
    let mut c = Client::start(tmp.path());
    let hits = c.call_json("search_components", json!({ "query": "date picker" }));
    assert_eq!(names(&hits)[0], "date_picker", "{:?}", names(&hits));
    let hits = c.call_json("search_components", json!({ "query": "modal" }));
    assert!(names(&hits).contains(&"dialog"), "{:?}", names(&hits));
    let hits = c.call_json(
        "search_components",
        json!({ "query": "button", "limit": 2 }),
    );
    assert_eq!(hits.len(), 2);
    assert_eq!(names(&hits)[0], "button");
    let (text, is_error) = c.call("search_components", json!({ "query": "zzqx" }));
    assert!(!is_error && text.starts_with("No component"), "{text}");
    let (_, is_error) = c.call("search_components", json!({}));
    assert!(is_error);
}

#[test]
fn get_component_is_the_docs_page() {
    let tmp = tempfile::tempdir().unwrap();
    let mut c = Client::start(tmp.path());
    let root = repo_root();
    let reg = atoll_cli::load(&root).unwrap();
    let expected = atoll_cli::llms::page(&root, &reg, "dialog")
        .unwrap()
        .unwrap();
    let (text, is_error) = c.call("get_component", json!({ "name": "dialog" }));
    assert!(!is_error);
    assert_eq!(text, expected);
    assert!(text.contains("## API") && text.contains("## Example"));

    let (text, _) = c.call(
        "get_component",
        json!({ "name": "badge", "include_source": true }),
    );
    assert!(text.contains("## Source"), "{text}");
    assert!(
        text.contains("### badge/component.rs\n\n```rust\n"),
        "{text}"
    );
    // Generated files are named, not dumped.
    let (text, _) = c.call(
        "get_component",
        json!({ "name": "icons", "include_source": true }),
    );
    assert!(text.contains("### icons/lucide.rs\n\nGenerated"), "{text}");

    let (text, is_error) = c.call("get_component", json!({ "name": "dialo" }));
    assert!(is_error && text.contains("did you mean"), "{text}");
}

#[test]
fn add_component_copies_into_the_project() {
    let tmp = tempfile::tempdir().unwrap();
    let mut c = Client::start(tmp.path());
    let (text, is_error) = c.call("add_component", json!({ "names": ["badge"] }));
    assert!(!is_error, "{text}");
    assert!(text.contains("pub mod badge;"), "{text}");
    let copied = tmp.path().join("src/components/badge/component.rs");
    let original = repo_root().join("crates/ui/src/components/badge/component.rs");
    assert_eq!(
        std::fs::read(&copied).unwrap(),
        std::fs::read(original).unwrap()
    );
    // Recorded for `atoll update`, with the folder relative to the project.
    let lock = atoll_cli::update::Lock::load(&tmp.path().join("atoll-lock.json")).unwrap();
    assert_eq!(lock.items["badge"].dir, "src/components");
    assert!(lock.items["badge"].files.contains_key("badge/component.rs"));
    // Edits are kept unless forced, like `atoll add`.
    std::fs::write(&copied, "// mine").unwrap();
    let (text, _) = c.call("add_component", json!({ "names": ["badge"] }));
    assert!(text.contains("Kept badge/component.rs"), "{text}");
    assert_eq!(std::fs::read_to_string(&copied).unwrap(), "// mine");
    c.call(
        "add_component",
        json!({ "names": ["badge"], "force": true }),
    );
    assert_ne!(std::fs::read_to_string(&copied).unwrap(), "// mine");

    // Blocks go to their own folder.
    let (text, is_error) = c.call(
        "add_component",
        json!({ "names": ["login_01"], "blocks_dest": "app/blocks" }),
    );
    assert!(!is_error, "{text}");
    assert!(tmp.path().join("app/blocks/login_01/block.rs").is_file());

    // Nothing is written outside the project.
    for dest in ["../escape", "/tmp/escape"] {
        let (text, is_error) = c.call("add_component", json!({ "names": ["badge"], "dest": dest }));
        assert!(is_error, "{dest}: {text}");
    }
    let (text, is_error) = c.call("add_component", json!({ "names": ["nope"] }));
    assert!(is_error && text.contains("unknown"), "{text}");
}

#[test]
fn init_theme_copies_the_stylesheet() {
    let tmp = tempfile::tempdir().unwrap();
    let mut c = Client::start(tmp.path());
    let (text, is_error) = c.call("init_theme", json!({}));
    assert!(
        !is_error && text.starts_with("Wrote assets/theme.css"),
        "{text}"
    );
    let theme = tmp.path().join("assets/theme.css");
    assert_eq!(
        std::fs::read(&theme).unwrap(),
        std::fs::read(repo_root().join(atoll_cli::THEME_FILE)).unwrap()
    );
    let (text, _) = c.call("init_theme", json!({}));
    assert!(text.contains("already up to date"), "{text}");
    std::fs::write(&theme, "/* mine */").unwrap();
    let (text, _) = c.call("init_theme", json!({}));
    assert!(text.starts_with("Kept"), "{text}");
}

#[test]
fn resources_serve_the_llms_pages() {
    let tmp = tempfile::tempdir().unwrap();
    let mut c = Client::start(tmp.path());
    let reply = c.request("resources/list", json!({}));
    let resources = reply["result"]["resources"].as_array().unwrap();
    assert_eq!(resources[0]["uri"], "atoll://llms.txt");
    assert!(resources.iter().any(|r| r["uri"] == "atoll://docs/dialog"));
    let reply = c.request("resources/read", json!({ "uri": "atoll://llms.txt" }));
    let text = reply["result"]["contents"][0]["text"].as_str().unwrap();
    assert_eq!(
        text,
        std::fs::read_to_string(repo_root().join("llms.txt")).unwrap()
    );
    let reply = c.request("resources/read", json!({ "uri": "atoll://docs/nope" }));
    assert!(reply["error"].is_object(), "{reply}");
}

#[test]
fn protocol_errors_and_notifications() {
    let tmp = tempfile::tempdir().unwrap();
    let mut c = Client::start(tmp.path());
    // A notification gets no reply, so the next line answers the next request.
    c.send(&json!({ "jsonrpc": "2.0", "method": "notifications/cancelled", "params": {} }));
    let reply = c.request("no/such/method", json!({}));
    assert_eq!(reply["error"]["code"], -32601);
    let reply = c.request("tools/call", json!({ "name": "nope" }));
    assert_eq!(reply["error"]["code"], -32602);
    writeln!(c.stdin, "not json").unwrap();
    c.stdin.flush().unwrap();
    let reply = c.read();
    assert_eq!(reply["error"]["code"], -32700);
    assert!(reply["id"].is_null());
}

#[test]
fn unknown_protocol_version_gets_the_newest() {
    let server = atoll_cli::mcp::Server::new(repo_root(), ".");
    let reply = server
        .handle_line(r#"{"jsonrpc":"2.0","id":"a","method":"initialize","params":{"protocolVersion":"1999-01-01"}}"#)
        .unwrap();
    let reply: Value = serde_json::from_str(&reply).unwrap();
    assert_eq!(reply["id"], "a");
    assert_eq!(
        reply["result"]["protocolVersion"],
        atoll_cli::mcp::PROTOCOL_VERSIONS[0]
    );
    let old = server
        .handle_line(r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05"}}"#)
        .unwrap();
    assert!(old.contains("\"protocolVersion\":\"2024-11-05\""));
}

#[test]
fn explain_errors_reads_cargo_output() {
    let tmp = tempfile::tempdir().unwrap();
    let mut c = Client::start(tmp.path());
    let fixture = repo_root().join("crates/cli/tests/fixtures/explain/cargo.jsonl");
    let output = std::fs::read_to_string(fixture).unwrap();
    let (text, is_error) = c.call("explain_errors", json!({ "cargo_output": output }));
    assert!(!is_error, "{text}");
    // The same plain-words report `atoll explain` prints.
    let expected = atoll_cli::explain::render(
        &atoll_cli::explain::Api::load(
            &repo_root(),
            &[
                tmp.path().join("src/components"),
                tmp.path().join("src/blocks"),
            ],
        )
        .explain(
            &std::fs::read_to_string(
                repo_root().join("crates/cli/tests/fixtures/explain/cargo.jsonl"),
            )
            .unwrap(),
            tmp.path(),
        ),
    );
    assert_eq!(text, expected);
    assert!(!text.is_empty());
    let (text, is_error) = c.call("explain_errors", json!({ "cargo_output": "" }));
    assert!(!is_error);
    assert!(text.starts_with("No errors"), "{text}");
}

#[test]
fn contracts_are_served() {
    let tmp = tempfile::tempdir().unwrap();
    let mut c = Client::start(tmp.path());
    let (text, is_error) = c.call("get_contract", json!({ "name": "dialog" }));
    assert!(!is_error, "{text}");
    assert!(text.contains("| Escape |"), "{text}");
    let (index, _) = c.call("get_contract", json!({}));
    assert!(
        index.contains("dialog") && index.contains("keyboard contract"),
        "{index}"
    );
    let (text, is_error) = c.call("get_contract", json!({ "name": "badge" }));
    assert!(!is_error && text.contains("no keyboard contract"), "{text}");
    let (text, is_error) = c.call("get_contract", json!({ "name": "dialgo" }));
    assert!(is_error, "{text}");
}

#[test]
fn themes_are_readable_in_both_modes() {
    let tmp = tempfile::tempdir().unwrap();
    let mut c = Client::start(tmp.path());
    let (text, is_error) = c.call(
        "generate_theme",
        json!({ "brand": "#facc15", "name": "acme" }),
    );
    assert!(!is_error, "{text}");
    assert!(text.contains("[data-accent=\"acme\"]"), "{text}");
    assert!(text.contains("--accent-color: light-dark(#"), "{text}");
    let (text, is_error) = c.call("generate_theme", json!({ "brand": "yellow" }));
    assert!(is_error, "{text}");
    let (text, is_error) = c.call(
        "generate_theme",
        json!({ "brand": "#fff", "name": "a\"]{" }),
    );
    assert!(is_error, "{text}");
}

#[test]
fn templates_match_the_cli() {
    let tmp = tempfile::tempdir().unwrap();
    let mut c = Client::start(tmp.path());
    let rows = c.call_json("list_templates", json!({}));
    let listed = names(&rows);
    let expected: Vec<&str> = atoll_cli::template::TEMPLATES
        .iter()
        .map(|t| t.name)
        .collect();
    assert_eq!(listed, expected);
    assert!(rows[0]["command"]
        .as_str()
        .unwrap()
        .contains("--template dashboard"));
}
