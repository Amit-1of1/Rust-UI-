//! `rust-ui mcp`: a [Model Context Protocol](https://modelcontextprotocol.io) server over stdio,
//! so coding agents can list, search, read and add registry items without a shell.
//!
//! One JSON-RPC 2.0 message per line on stdin, one reply per line on stdout (the MCP stdio
//! transport); nothing else is written to stdout. The tools are thin wrappers over the same
//! functions as `rust-ui list`, `docs`, `add` and `init`, so the answers match the CLI.
//! The pages are also served as resources (`rust-ui://llms.txt`, `rust-ui://docs/<name>`).

use crate::docsgen::title;
use crate::llms::{index, page};
use crate::manifest::description;
use crate::{add_to, compare, load, mod_lines_of, resolve, FileState, Item, Kind, Registry};
use crate::{THEME_FILE, VERSION};
use serde_json::{json, Value};
use std::fs;
use std::io::{self, BufRead, Write};
use std::path::{Component, Path, PathBuf};

/// Protocol revisions this server speaks, newest first. A client asking for another one gets
/// the newest, as the spec says.
pub const PROTOCOL_VERSIONS: &[&str] = &["2025-06-18", "2025-03-26", "2024-11-05"];

const PARSE_ERROR: i64 = -32700;
const INVALID_REQUEST: i64 = -32600;
const METHOD_NOT_FOUND: i64 = -32601;
const INVALID_PARAMS: i64 = -32602;

const INDEX_URI: &str = "rust-ui://llms.txt";
const DOCS_URI: &str = "rust-ui://docs/";

pub struct Server {
    /// The Rust UI checkout components are read from (`--source`).
    root: PathBuf,
    /// The app `add_component` and `init_theme` write into (`--project`).
    project: PathBuf,
}

impl Server {
    pub fn new(root: impl Into<PathBuf>, project: impl Into<PathBuf>) -> Self {
        Self {
            root: root.into(),
            project: project.into(),
        }
    }

    /// Answer messages from `input` until it closes.
    pub fn serve(&self, input: impl BufRead, mut output: impl Write) -> io::Result<()> {
        for line in input.lines() {
            let line = line?;
            if line.trim().is_empty() {
                continue;
            }
            if let Some(reply) = self.handle_line(&line) {
                writeln!(output, "{reply}")?;
                output.flush()?;
            }
        }
        Ok(())
    }

    /// The reply to one line, or `None` for notifications.
    pub fn handle_line(&self, line: &str) -> Option<String> {
        let reply = match serde_json::from_str::<Value>(line) {
            Err(e) => Some(error(Value::Null, PARSE_ERROR, &e.to_string())),
            // Batches are from 2025-03-26 only, but cost nothing to accept.
            Ok(Value::Array(batch)) => {
                let replies: Vec<Value> = batch.iter().filter_map(|m| self.handle(m)).collect();
                (!replies.is_empty()).then_some(Value::Array(replies))
            }
            Ok(message) => self.handle(&message),
        };
        reply.map(|r| r.to_string())
    }

    fn handle(&self, message: &Value) -> Option<Value> {
        let id = message.get("id").cloned();
        let Some(method) = message.get("method").and_then(Value::as_str) else {
            // A response to us (we send no requests) or junk.
            return id.map(|id| error(id, INVALID_REQUEST, "expected a request"));
        };
        // Notifications (`notifications/initialized`, `notifications/cancelled`) need no reply.
        let id = id?;
        let params = message.get("params").cloned().unwrap_or(Value::Null);
        Some(match self.request(method, &params) {
            Ok(result) => json!({ "jsonrpc": "2.0", "id": id, "result": result }),
            Err((code, msg)) => error(id, code, &msg),
        })
    }

    fn request(&self, method: &str, params: &Value) -> Result<Value, (i64, String)> {
        match method {
            "initialize" => {
                let asked = params.get("protocolVersion").and_then(Value::as_str);
                let version = asked
                    .filter(|v| PROTOCOL_VERSIONS.contains(v))
                    .unwrap_or(PROTOCOL_VERSIONS[0]);
                Ok(json!({
                    "protocolVersion": version,
                    "capabilities": { "tools": {}, "resources": {} },
                    "serverInfo": { "name": "rust-ui", "title": "Rust UI registry", "version": VERSION },
                    "instructions": INSTRUCTIONS,
                }))
            }
            "ping" => Ok(json!({})),
            "tools/list" => Ok(json!({ "tools": tools() })),
            "tools/call" => {
                let name = params
                    .get("name")
                    .and_then(Value::as_str)
                    .ok_or((INVALID_PARAMS, "missing tool `name`".to_string()))?;
                let args = params.get("arguments").cloned().unwrap_or(json!({}));
                if !tools().iter().any(|t| t["name"] == name) {
                    return Err((INVALID_PARAMS, format!("unknown tool `{name}`")));
                }
                // Failures inside a tool are results the model can read, not protocol errors.
                Ok(match self.call(name, &args) {
                    Ok(text) => json!({ "content": [{ "type": "text", "text": text }] }),
                    Err(text) => {
                        json!({ "content": [{ "type": "text", "text": text }], "isError": true })
                    }
                })
            }
            "resources/list" => self.resources().map_err(internal),
            "resources/read" => {
                let uri = params
                    .get("uri")
                    .and_then(Value::as_str)
                    .ok_or((INVALID_PARAMS, "missing `uri`".to_string()))?;
                self.read_resource(uri)
            }
            "resources/templates/list" => Ok(json!({ "resourceTemplates": [{
                "uriTemplate": format!("{DOCS_URI}{{name}}"),
                "name": "component-docs",
                "title": "Component page",
                "description": "Docs, API and example for one component or block",
                "mimeType": "text/markdown",
            }] })),
            _ => Err((METHOD_NOT_FOUND, format!("unknown method `{method}`"))),
        }
    }

    fn registry(&self) -> Result<Registry, String> {
        load(&self.root)
    }

    fn call(&self, tool: &str, args: &Value) -> Result<String, String> {
        match tool {
            "list_components" => {
                let kind = optional_str(args, "kind")?;
                if let Some(k) = kind.filter(|k| !["all", "component", "block"].contains(k)) {
                    return Err(format!(
                        "`kind` must be `component`, `block` or `all`, not `{k}`"
                    ));
                }
                let reg = self.registry()?;
                let rows: Vec<Value> = reg
                    .items
                    .iter()
                    .filter(|i| match kind {
                        Some("component") => i.kind == Kind::Component,
                        Some("block") => i.kind == Kind::Block,
                        _ => true,
                    })
                    .map(|i| self.summary(i))
                    .collect();
                Ok(pretty(&rows))
            }
            "search_components" => {
                let query = required_str(args, "query")?;
                let limit = args.get("limit").and_then(Value::as_u64).unwrap_or(10) as usize;
                let reg = self.registry()?;
                let hits = self.search(&reg, query);
                if hits.is_empty() {
                    return Ok(format!(
                        "No component or block matches `{query}`. `list_components` shows all {}.",
                        reg.items.len()
                    ));
                }
                let rows: Vec<Value> = hits
                    .into_iter()
                    .take(limit.max(1))
                    .map(|i| self.summary(i))
                    .collect();
                Ok(pretty(&rows))
            }
            "get_component" => {
                let name = required_str(args, "name")?;
                let reg = self.registry()?;
                let mut text = page(&self.root, &reg, name)
                    .map_err(|e| e.to_string())?
                    .ok_or_else(|| unknown(&reg, name))?;
                if args.get("include_source").and_then(Value::as_bool) == Some(true) {
                    let item = reg
                        .items
                        .iter()
                        .find(|i| i.name == name)
                        .expect("page found");
                    text.push_str("\n## Source\n");
                    for file in &item.files {
                        let path = self.root.join(item.kind.dir()).join(file);
                        let body = fs::read_to_string(&path).map_err(|e| e.to_string())?;
                        if body.starts_with("// @generated") {
                            text.push_str(&format!(
                                "\n### {file}\n\nGenerated, {} lines; not shown.\n",
                                body.lines().count()
                            ));
                            continue;
                        }
                        let lang = match Path::new(file).extension().and_then(|e| e.to_str()) {
                            Some("rs") => "rust",
                            Some("css") => "css",
                            Some("md") => "markdown",
                            _ => "",
                        };
                        text.push_str(&format!(
                            "\n### {file}\n\n```{lang}\n{}\n```\n",
                            body.trim_end()
                        ));
                    }
                }
                Ok(text)
            }
            "add_component" => {
                let names: Vec<String> = match args.get("names") {
                    Some(Value::Array(a)) => a
                        .iter()
                        .map(|v| v.as_str().map(String::from))
                        .collect::<Option<_>>()
                        .ok_or("`names` must be a list of strings")?,
                    Some(Value::String(s)) => vec![s.clone()],
                    _ => {
                        return Err("`names` is required: a list of component or block names".into())
                    }
                };
                if names.is_empty() {
                    return Err("`names` is empty".into());
                }
                let dest = self.inside(optional_str(args, "dest")?.unwrap_or("src/components"))?;
                let blocks =
                    self.inside(optional_str(args, "blocks_dest")?.unwrap_or("src/blocks"))?;
                let force = args.get("force").and_then(Value::as_bool).unwrap_or(false);
                let reg = self.registry()?;
                let items = resolve(&reg, &names)?;
                let (written, kept) =
                    add_to(&self.root, &items, &dest, &blocks, force).map_err(|e| e.to_string())?;
                let mut out = format!(
                    "Added {} (wrote {} file(s)).\n",
                    items
                        .iter()
                        .map(|i| format!("`{}`", i.name))
                        .collect::<Vec<_>>()
                        .join(", "),
                    written.len()
                );
                for k in &kept {
                    out.push_str(&format!(
                        "Kept {} (it differs from the registry; pass `force` to overwrite).\n",
                        k.display()
                    ));
                }
                for (kind, dir) in [(Kind::Component, &dest), (Kind::Block, &blocks)] {
                    let lines = mod_lines_of(&items, kind);
                    if !lines.is_empty() {
                        let rel = dir.strip_prefix(&self.project).unwrap_or(dir);
                        out.push_str(&format!(
                            "\nMake sure {}/mod.rs declares:\n{lines}",
                            rel.display()
                        ));
                    }
                }
                out.push_str(&format!(
                    "\nThe app needs `{}` in Cargo.toml and the theme stylesheet (`init_theme`).\n",
                    reg.cargo_dependencies.join("`, `")
                ));
                Ok(out)
            }
            "init_theme" => {
                let dest =
                    self.inside(optional_str(args, "dest")?.unwrap_or("assets/theme.css"))?;
                let force = args.get("force").and_then(Value::as_bool).unwrap_or(false);
                let from = self.root.join(THEME_FILE);
                let rel = dest.strip_prefix(&self.project).unwrap_or(&dest).display();
                match compare(&from, &dest).map_err(|e| e.to_string())? {
                    FileState::Same => return Ok(format!("{rel} is already up to date.")),
                    FileState::Changed if !force => {
                        return Ok(format!(
                            "Kept {rel} (it differs from the registry's theme; pass `force` to overwrite)."
                        ))
                    }
                    _ => {}
                }
                if let Some(parent) = dest.parent() {
                    fs::create_dir_all(parent).map_err(|e| e.to_string())?;
                }
                fs::copy(&from, &dest).map_err(|e| e.to_string())?;
                Ok(format!("Wrote {rel}. Link it from the app, e.g. `document::Stylesheet {{ href: asset!(\"/{rel}\") }}`."))
            }
            _ => unreachable!("checked against tools()"),
        }
    }

    /// A path the client gave, resolved inside the project. Absolute paths and `..` are
    /// refused so a tool call can't write outside the app it was started for.
    fn inside(&self, path: &str) -> Result<PathBuf, String> {
        let p = Path::new(path);
        if p.components()
            .any(|c| !matches!(c, Component::Normal(_) | Component::CurDir))
        {
            return Err(format!(
                "`{path}` must be a relative path inside the project (no `..`, not absolute)"
            ));
        }
        Ok(self.project.join(p))
    }

    fn docs(&self, item: &Item) -> String {
        fs::read_to_string(
            self.root
                .join(item.kind.dir())
                .join(&item.name)
                .join("docs.md"),
        )
        .unwrap_or_default()
    }

    fn summary(&self, item: &Item) -> Value {
        let docs = self.docs(item);
        json!({
            "name": item.name,
            "kind": match item.kind { Kind::Component => "component", Kind::Block => "block" },
            "title": title(&docs, &item.name),
            "description": description(&docs),
            "dependencies": item.registry_dependencies,
        })
    }

    /// Items matching every word of `query`, best first: a word in the name counts most, then
    /// the title, the first paragraph, and the rest of `docs.md`.
    fn search<'a>(&self, reg: &'a Registry, query: &str) -> Vec<&'a Item> {
        let words: Vec<String> = query
            .split(|c: char| !c.is_alphanumeric())
            .filter(|w| !w.is_empty())
            .map(str::to_lowercase)
            .collect();
        let mut scored: Vec<(u32, &Item)> = reg
            .items
            .iter()
            .filter_map(|item| {
                let docs = self.docs(item);
                let name = item.name.to_lowercase();
                let title = title(&docs, &item.name).to_lowercase();
                let summary = description(&docs).to_lowercase();
                let body = docs.to_lowercase();
                let mut total = 0;
                for w in &words {
                    let score = if name == *w || name.replace('_', "") == *w {
                        100
                    } else if name.split('_').any(|part| part == w) {
                        60
                    } else if name.contains(w.as_str()) {
                        40
                    } else if title.contains(w.as_str()) {
                        30
                    } else if summary.contains(w.as_str()) {
                        15
                    } else if body.contains(w.as_str()) {
                        5
                    } else {
                        return None;
                    };
                    total += score;
                }
                Some((total, item))
            })
            .collect();
        scored.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.name.cmp(&b.1.name)));
        scored.into_iter().map(|(_, i)| i).collect()
    }

    fn resources(&self) -> io::Result<Value> {
        let reg = self.registry().map_err(io::Error::other)?;
        let mut list = vec![json!({
            "uri": INDEX_URI,
            "name": "llms.txt",
            "title": "Rust UI index",
            "description": "What the kit is, the rules for using it, and one line per component",
            "mimeType": "text/markdown",
        })];
        for item in &reg.items {
            let docs = self.docs(item);
            list.push(json!({
                "uri": format!("{DOCS_URI}{}", item.name),
                "name": item.name,
                "title": title(&docs, &item.name),
                "description": description(&docs),
                "mimeType": "text/markdown",
            }));
        }
        Ok(json!({ "resources": list }))
    }

    fn read_resource(&self, uri: &str) -> Result<Value, (i64, String)> {
        let reg = self.registry().map_err(|e| (INVALID_PARAMS, e))?;
        let text = if uri == INDEX_URI {
            index(&self.root, &reg).map_err(internal)?
        } else if let Some(name) = uri.strip_prefix(DOCS_URI) {
            page(&self.root, &reg, name)
                .map_err(internal)?
                .ok_or_else(|| (INVALID_PARAMS, unknown(&reg, name)))?
        } else {
            return Err((INVALID_PARAMS, format!("unknown resource `{uri}`")));
        };
        Ok(json!({ "contents": [{ "uri": uri, "mimeType": "text/markdown", "text": text }] }))
    }
}

const INSTRUCTIONS: &str = "Rust UI is a shadcn-style component kit for Dioxus 0.7. Components are copied into the app as source. Before writing UI code, call search_components or list_components, then get_component for the exact props and an example; prefer kit components over hand-written markup. add_component copies a component and the components it needs into the project and lists the `pub mod` lines to add.";

fn tools() -> Vec<Value> {
    let read_only = json!({ "readOnlyHint": true, "openWorldHint": false });
    vec![
        json!({
            "name": "list_components",
            "title": "List components",
            "description": "Every component and block in the Rust UI registry, with a one-line description and the components each one depends on.",
            "inputSchema": { "type": "object", "properties": {
                "kind": { "type": "string", "enum": ["component", "block", "all"], "description": "Only components or only blocks (whole screens). Default: all." }
            } },
            "annotations": read_only,
        }),
        json!({
            "name": "search_components",
            "title": "Search components",
            "description": "Find components and blocks by words in their name, title or docs (e.g. `date picker`, `modal`, `table sort`). Best matches first.",
            "inputSchema": { "type": "object", "properties": {
                "query": { "type": "string", "description": "Words to look for; every word must match." },
                "limit": { "type": "integer", "minimum": 1, "description": "Most results to return. Default: 10." }
            }, "required": ["query"] },
            "annotations": read_only,
        }),
        json!({
            "name": "get_component",
            "title": "Read a component's docs",
            "description": "The page for one component or block: how to install and import it, its docs, the Rust signatures of its props and a working example. Same text as `rust-ui docs <name>`.",
            "inputSchema": { "type": "object", "properties": {
                "name": { "type": "string", "description": "Registry name, e.g. `dialog` or `login_01`." },
                "include_source": { "type": "boolean", "description": "Also return every file of the item in full. Default: false." }
            }, "required": ["name"] },
            "annotations": read_only,
        }),
        json!({
            "name": "add_component",
            "title": "Add components to the project",
            "description": "Copy components or blocks, and every component they depend on, into the project (like `rust-ui add`). Files the user has edited are kept unless `force` is set.",
            "inputSchema": { "type": "object", "properties": {
                "names": { "type": "array", "items": { "type": "string" }, "minItems": 1, "description": "Registry names to add." },
                "dest": { "type": "string", "description": "Components folder, relative to the project. Default: src/components." },
                "blocks_dest": { "type": "string", "description": "Blocks folder, relative to the project. Default: src/blocks." },
                "force": { "type": "boolean", "description": "Overwrite files that differ from the registry. Default: false." }
            }, "required": ["names"] },
            "annotations": { "readOnlyHint": false, "destructiveHint": false, "idempotentHint": true, "openWorldHint": false },
        }),
        json!({
            "name": "init_theme",
            "title": "Add the theme stylesheet",
            "description": "Copy the kit's theme CSS (colours, light and dark mode, spacing, motion tokens) into the project, like `rust-ui init`. Every component needs it.",
            "inputSchema": { "type": "object", "properties": {
                "dest": { "type": "string", "description": "Destination, relative to the project. Default: assets/theme.css." },
                "force": { "type": "boolean", "description": "Overwrite a theme file that differs. Default: false." }
            } },
            "annotations": { "readOnlyHint": false, "destructiveHint": false, "idempotentHint": true, "openWorldHint": false },
        }),
    ]
}

fn error(id: Value, code: i64, message: &str) -> Value {
    json!({ "jsonrpc": "2.0", "id": id, "error": { "code": code, "message": message } })
}

fn internal(e: io::Error) -> (i64, String) {
    (-32603, e.to_string())
}

fn unknown(reg: &Registry, name: &str) -> String {
    let close: Vec<&str> = reg
        .items
        .iter()
        .map(|i| i.name.as_str())
        .filter(|n| n.contains(name) || name.contains(*n))
        .take(5)
        .collect();
    if close.is_empty() {
        format!("unknown component or block `{name}`; try search_components")
    } else {
        format!(
            "unknown component or block `{name}`; did you mean {}?",
            close.join(", ")
        )
    }
}

fn required_str<'a>(args: &'a Value, key: &str) -> Result<&'a str, String> {
    optional_str(args, key)?.ok_or_else(|| format!("`{key}` is required"))
}

fn optional_str<'a>(args: &'a Value, key: &str) -> Result<Option<&'a str>, String> {
    match args.get(key) {
        None | Some(Value::Null) => Ok(None),
        Some(Value::String(s)) => Ok(Some(s)),
        Some(_) => Err(format!("`{key}` must be a string")),
    }
}

fn pretty(v: &impl serde::Serialize) -> String {
    serde_json::to_string_pretty(v).expect("serialises")
}
