//! Let an AI coding agent see and drive your app: a Model Context Protocol (MCP) server over
//! stdio that renders a screen headless (the same [`Screen`] the tests use) and gives the
//! agent tools to read it, click, type, press keys, let time pass and take screenshots.
//!
//! Agents are poor at guessing what a UI looks like from its source. With this they read the
//! real screen, act on it like a user, and check their own work without a browser, a window
//! or a person in the loop. It runs anywhere `cargo test` runs, CI and containers included.
//!
//! Add a binary (or an example) to the app:
//!
//! ```no_run
//! // examples/agent.rs
//! use dioxus::prelude::*;
//!
//! fn app() -> Element {
//!     rsx! { button { "Hello" } }
//! }
//!
//! fn main() {
//!     atoll_test::agent::serve(atoll_test::Screen::builder(app));
//! }
//! ```
//!
//! and register `cargo run --example agent` as an MCP server in the agent (for Claude Code:
//! `claude mcp add my-app -- cargo run --example agent`). docs/AGENTS.md has the details.
//!
//! # Stability
//!
//! Experimental (docs/STABILITY.md): new on 2026-11-15; tool names may still change.

use crate::query::{css, label, placeholder, role, test_id, text, text_containing, Query};
use crate::{Screen, ScreenBuilder};
use serde_json::{json, Value};
use std::io::{BufRead, Write};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::time::Duration;

const PROTOCOL: &str = "2025-06-18";

/// Render the screen and answer MCP requests on stdin/stdout until stdin closes.
pub fn serve(builder: ScreenBuilder) {
    let mut agent = Agent::new(builder.render());
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout().lock();
    for line in stdin.lock().lines() {
        let Ok(line) = line else { break };
        if line.trim().is_empty() {
            continue;
        }
        let reply = match serde_json::from_str::<Value>(&line) {
            Ok(msg) => agent.handle(&msg),
            Err(e) => Some(
                json!({ "jsonrpc": "2.0", "id": null, "error": { "code": -32700, "message": e.to_string() } }),
            ),
        };
        if let Some(reply) = reply {
            let _ = writeln!(stdout, "{reply}");
            let _ = stdout.flush();
        }
    }
}

/// The server's state: one live screen. Public so an app can drive it from its own transport
/// (or a test); [`serve`] is the stdio loop around it.
pub struct Agent {
    screen: Screen,
}

impl Agent {
    pub fn new(screen: Screen) -> Self {
        Self { screen }
    }

    /// The screen, for a test that wants to look after a tool call.
    pub fn screen(&self) -> &Screen {
        &self.screen
    }

    /// Answer one JSON-RPC message; `None` for notifications.
    pub fn handle(&mut self, msg: &Value) -> Option<Value> {
        let id = msg.get("id").cloned();
        let method = msg.get("method").and_then(Value::as_str).unwrap_or("");
        let id = id?; // Notifications (no id) get no reply.
        let result = match method {
            "initialize" => Ok(json!({
                "protocolVersion": PROTOCOL,
                "capabilities": { "tools": {} },
                "serverInfo": { "name": "atoll-screen", "title": "Atoll app screen", "version": env!("CARGO_PKG_VERSION") },
                "instructions": "A headless copy of the app. Call `outline` to see the screen as a user would (roles, names, state), then act with `click`, `fill`, `press` and check again. `screenshot` returns a PNG. Find elements by role+name (best), label, placeholder, text, test_id or css.",
            })),
            "ping" => Ok(json!({})),
            "tools/list" => Ok(json!({ "tools": tools() })),
            "tools/call" => {
                let name = msg["params"]["name"].as_str().unwrap_or("");
                let args = msg["params"].get("arguments").cloned().unwrap_or(json!({}));
                Ok(match self.call(name, &args) {
                    Ok(content) => json!({ "content": content }),
                    Err(text) => {
                        json!({ "content": [{ "type": "text", "text": text }], "isError": true })
                    }
                })
            }
            _ => Err((-32601, format!("unknown method `{method}`"))),
        };
        Some(match result {
            Ok(r) => json!({ "jsonrpc": "2.0", "id": id, "result": r }),
            Err((code, message)) => {
                json!({ "jsonrpc": "2.0", "id": id, "error": { "code": code, "message": message } })
            }
        })
    }

    /// Run one tool. Screen methods panic on a bad query (that's what tests want); here the
    /// panic message goes back to the agent instead.
    pub fn call(&mut self, name: &str, args: &Value) -> Result<Vec<Value>, String> {
        let screen = &mut self.screen;
        let outcome = catch_unwind(AssertUnwindSafe(|| -> Result<Vec<Value>, String> {
            let say = |t: String| vec![json!({ "type": "text", "text": t })];
            match name {
                "outline" => Ok(say(screen.outline())),
                "click" => {
                    let q = query(args)?;
                    let el = screen.find(q.clone())?;
                    let what = screen.describe(el);
                    screen.click(q);
                    Ok(say(format!("Clicked {what}.\n\n{}", screen.outline())))
                }
                "fill" => {
                    let q = query(args)?;
                    let value = args["value"]
                        .as_str()
                        .ok_or("`value` (the text to put in) is required")?;
                    screen.find(q.clone())?;
                    screen.fill(q, value);
                    Ok(say(format!(
                        "Filled it with {value:?}.\n\n{}",
                        screen.outline()
                    )))
                }
                "press" => {
                    let key = args["key"].as_str().ok_or(
                        "`key` is required, e.g. `Enter`, `Tab`, `Escape`, `ArrowDown`, `Ctrl+K`",
                    )?;
                    let times = args["times"].as_u64().unwrap_or(1).max(1) as usize;
                    screen.press_times(key, times);
                    let focus = screen
                        .focused()
                        .map(|e| screen.describe(e))
                        .unwrap_or_else(|| "nothing".into());
                    Ok(say(format!(
                        "Pressed {key} x{times}. Focus is on {focus}.\n\n{}",
                        screen.outline()
                    )))
                }
                "type_text" => {
                    let t = args["text"].as_str().ok_or("`text` is required")?;
                    screen.type_text(t);
                    Ok(say(format!("Typed {t:?}.\n\n{}", screen.outline())))
                }
                "wait" => {
                    let ms = args["ms"].as_u64().unwrap_or(500);
                    screen.advance(Duration::from_millis(ms));
                    Ok(say(format!("{ms} ms passed.\n\n{}", screen.outline())))
                }
                "read" => {
                    let q = query(args)?;
                    let el = screen.find(q.clone())?;
                    let r = screen.rect(q.clone());
                    Ok(say(format!(
                        "{}\nvisible: {}\nbox: x {:.0}, y {:.0}, {:.0} x {:.0}\nhtml: {}",
                        screen.describe(el),
                        screen.is_visible(q.clone()),
                        r.x,
                        r.y,
                        r.w,
                        r.h,
                        screen.html(q).chars().take(2000).collect::<String>()
                    )))
                }
                "screenshot" => {
                    let png = screen.png();
                    let (w, h) = screen.size();
                    Ok(vec![
                        json!({ "type": "image", "data": base64(&png), "mimeType": "image/png" }),
                        json!({ "type": "text", "text": format!("{w} x {h} screenshot.") }),
                    ])
                }
                _ => Err(format!("unknown tool `{name}`")),
            }
        }));
        match outcome {
            Ok(r) => r,
            Err(p) => Err(p
                .downcast_ref::<String>()
                .cloned()
                .or_else(|| p.downcast_ref::<&str>().map(|s| s.to_string()))
                .unwrap_or_else(|| "the action failed".into())),
        }
    }
}

/// Which element a tool means, from its arguments.
fn query(args: &Value) -> Result<Query, String> {
    let s = |k: &str| args.get(k).and_then(Value::as_str);
    let mut q = if let Some(r) = s("role") {
        let q = role(r);
        match s("name") {
            Some(n) => q.name(n),
            None => q,
        }
    } else if let Some(l) = s("label") {
        label(l)
    } else if let Some(p) = s("placeholder") {
        placeholder(p)
    } else if let Some(t) = s("text") {
        text(t)
    } else if let Some(t) = s("text_containing") {
        text_containing(t)
    } else if let Some(t) = s("test_id") {
        test_id(t)
    } else if let Some(c) = s("css") {
        css(c)
    } else {
        return Err("say which element: `role` (+ `name`), `label`, `placeholder`, `text`, `text_containing`, `test_id` or `css`".into());
    };
    if let Some(n) = args.get("nth").and_then(Value::as_u64) {
        q = q.nth(n as usize);
    }
    Ok(q)
}

fn target_schema(extra: Value) -> Value {
    let mut props = json!({
        "role": { "type": "string", "description": "ARIA role, e.g. `button`, `link`, `textbox`, `checkbox`, `tab`, `menuitem`, `dialog`." },
        "name": { "type": "string", "description": "With `role`: the accessible name (exact, case and spacing ignored)." },
        "label": { "type": "string", "description": "A form field by its label text." },
        "placeholder": { "type": "string" },
        "text": { "type": "string", "description": "An element by its exact text." },
        "text_containing": { "type": "string" },
        "test_id": { "type": "string", "description": "`data-testid`." },
        "css": { "type": "string", "description": "A CSS selector (last resort)." },
        "nth": { "type": "integer", "minimum": 0, "description": "Pick the nth match when several match." }
    });
    if let (Some(p), Some(e)) = (props.as_object_mut(), extra.as_object()) {
        for (k, v) in e {
            p.insert(k.clone(), v.clone());
        }
    }
    json!({ "type": "object", "properties": props })
}

fn tools() -> Vec<Value> {
    let read_only = json!({ "readOnlyHint": true, "openWorldHint": false });
    let acts = json!({ "readOnlyHint": false, "destructiveHint": false, "openWorldHint": false });
    vec![
        json!({
            "name": "outline",
            "title": "Read the screen",
            "description": "The screen as a user or screen reader meets it: one line per element with a role (role, name, value, state such as [expanded=true] or [focused]), indented by nesting, plus loose text. Hidden parts are left out. Start here.",
            "inputSchema": { "type": "object", "properties": {} },
            "annotations": read_only,
        }),
        json!({
            "name": "click",
            "title": "Click an element",
            "description": "Click the middle of an element, as a user would (it must be visible). Returns the new outline.",
            "inputSchema": target_schema(json!({})),
            "annotations": acts,
        }),
        json!({
            "name": "fill",
            "title": "Fill a field",
            "description": "Replace a text field's content with `value`. Returns the new outline.",
            "inputSchema": target_schema(json!({ "value": { "type": "string" } })),
            "annotations": acts,
        }),
        json!({
            "name": "press",
            "title": "Press a key",
            "description": "Press a key on whatever has focus: `Enter`, `Tab`, `Shift+Tab`, `Escape`, `ArrowDown`, `Home`, `Ctrl+K`, a letter. Returns where focus went and the new outline.",
            "inputSchema": { "type": "object", "properties": {
                "key": { "type": "string" },
                "times": { "type": "integer", "minimum": 1 }
            }, "required": ["key"] },
            "annotations": acts,
        }),
        json!({
            "name": "type_text",
            "title": "Type text",
            "description": "Type text into whatever has focus, key by key.",
            "inputSchema": { "type": "object", "properties": { "text": { "type": "string" } }, "required": ["text"] },
            "annotations": acts,
        }),
        json!({
            "name": "wait",
            "title": "Let time pass",
            "description": "Move the app's clock on (timers, debounces, toasts closing). Default 500 ms.",
            "inputSchema": { "type": "object", "properties": { "ms": { "type": "integer", "minimum": 0 } } },
            "annotations": acts,
        }),
        json!({
            "name": "read",
            "title": "Inspect an element",
            "description": "One element's description, whether it is visible, its box on screen and its HTML.",
            "inputSchema": target_schema(json!({})),
            "annotations": read_only,
        }),
        json!({
            "name": "screenshot",
            "title": "Take a screenshot",
            "description": "A PNG of the current frame (painted on the CPU, the app's real CSS).",
            "inputSchema": { "type": "object", "properties": {} },
            "annotations": read_only,
        }),
    ]
}

/// Standard base64 (RFC 4648, with padding).
fn base64(bytes: &[u8]) -> String {
    const T: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let b = [
            chunk[0],
            *chunk.get(1).unwrap_or(&0),
            *chunk.get(2).unwrap_or(&0),
        ];
        let n = (u32::from(b[0]) << 16) | (u32::from(b[1]) << 8) | u32::from(b[2]);
        out.push(T[(n >> 18) as usize & 63] as char);
        out.push(T[(n >> 12) as usize & 63] as char);
        out.push(if chunk.len() > 1 {
            T[(n >> 6) as usize & 63] as char
        } else {
            '='
        });
        out.push(if chunk.len() > 2 {
            T[n as usize & 63] as char
        } else {
            '='
        });
    }
    out
}

#[cfg(test)]
mod tests {
    use super::base64;

    #[test]
    fn base64_matches_rfc_4648() {
        assert_eq!(base64(b""), "");
        assert_eq!(base64(b"f"), "Zg==");
        assert_eq!(base64(b"fo"), "Zm8=");
        assert_eq!(base64(b"foo"), "Zm9v");
        assert_eq!(base64(b"foobar"), "Zm9vYmFy");
    }
}
