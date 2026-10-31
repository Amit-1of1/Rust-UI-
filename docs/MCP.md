# MCP server

`rust-ui mcp` serves the registry to AI coding tools over the [Model Context Protocol](https://modelcontextprotocol.io) (stdio). With it, an agent working in your Dioxus app can find a component, read its props and example, and copy it into the app without you pasting docs or running commands.

It is the same CLI as the rest of `rust-ui`, so install that first:

```sh
cargo install --git https://github.com/Amit-1of1/Rust-UI- rust-ui-cli   # installs `rust-ui`
```

## What the agent gets

| Tool | Does | Same as |
| --- | --- | --- |
| `list_components` | Every component and block, with a one-line description and its dependencies. `kind`: `component`, `block` or `all`. | `rust-ui list` |
| `search_components` | Components matching every word of `query`, names first, then titles and docs. `limit` defaults to 10. | |
| `get_component` | One item's page: install and import lines, docs, prop signatures and an example. `include_source: true` adds every file. | `rust-ui docs <name>` |
| `add_component` | Copies `names` and everything they depend on into `dest` (default `src/components`) and blocks into `blocks_dest` (default `src/blocks`). Keeps files you edited unless `force`. Replies with the `pub mod` lines to add. | `rust-ui add` |
| `init_theme` | Copies the theme stylesheet to `dest` (default `assets/theme.css`). | `rust-ui init` |

The pages are also resources, for clients that let you attach them: `rust-ui://llms.txt` (the index) and `rust-ui://docs/<name>`.

`add_component` and `init_theme` write only inside the project: paths are relative to it, and absolute paths or `..` are refused. The project is the folder the client starts the server in, or `--project <dir>`.

## Where components come from

`--source` works as for every other command: a git URL is shallow-cloned into `~/.cache/rust-ui` and updated each time the server starts (it needs `git`); a local checkout is read as it is, which is how you pin a version. `RUST_UI_SOURCE` works too.

## Claude Code

From your app's folder:

```sh
claude mcp add rust-ui -- rust-ui --source https://github.com/Amit-1of1/Rust-UI- mcp
```

Add `--scope project` to write it to `.mcp.json` so everyone on the project gets it. The file looks like this:

```json
{
  "mcpServers": {
    "rust-ui": {
      "command": "rust-ui",
      "args": ["--source", "https://github.com/Amit-1of1/Rust-UI-", "mcp"]
    }
  }
}
```

`claude mcp list` should show `rust-ui: ... - ✓ Connected`.

## Cursor

The same `mcpServers` block as above, in `.cursor/mcp.json` in the project (or `~/.cursor/mcp.json` for every project).

## VS Code (Copilot agent mode)

`.vscode/mcp.json`:

```json
{
  "servers": {
    "rust-ui": {
      "type": "stdio",
      "command": "rust-ui",
      "args": ["--source", "https://github.com/Amit-1of1/Rust-UI-", "mcp"]
    }
  }
}
```

## Codex CLI

`~/.codex/config.toml`:

```toml
[mcp_servers.rust-ui]
command = "rust-ui"
args = ["--source", "https://github.com/Amit-1of1/Rust-UI-", "mcp"]
```

## Claude Desktop and other clients without a project folder

Clients that don't start servers in your project need `--project` with an absolute path, otherwise `add_component` writes into whatever folder the client runs in. In `claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "rust-ui": {
      "command": "rust-ui",
      "args": ["--source", "https://github.com/Amit-1of1/Rust-UI-", "mcp", "--project", "/path/to/your/app"]
    }
  }
}
```

If the client can't find `rust-ui`, give `command` the full path (`~/.cargo/bin/rust-ui`, expanded).

## Agent skill and `rust-ui info`

MCP gives the agent tools; the skill tells it how to use the kit well. `skills/rust-ui/SKILL.md` is an [Agent Skill](https://agentskills.io): when to use the kit, to read the project with `rust-ui info --json` first, which component fits which need, the accessibility and motion rules, and to update with `rust-ui update` instead of `add --force`. Install it into your app:

```sh
rust-ui --source https://github.com/Amit-1of1/Rust-UI- skill            # writes .claude/skills/rust-ui/SKILL.md
rust-ui --source https://github.com/Amit-1of1/Rust-UI- skill --dest .agents/skills/rust-ui/SKILL.md
```

`rust-ui info` (`--json` for agents) reads only your app: the `dioxus` requirement in Cargo.toml, `assets/theme.css` and its accent presets, every item in `rust-ui-lock.json` with the files you edited, files still holding update conflict markers, and whether the skill and an MCP config are present. `crates/cli/tests/info.rs` covers it, and checks that every component the skill names exists in the registry.

## Trying it by hand

The server reads one JSON-RPC message per line and answers on stdout:

```sh
printf '%s\n' \
  '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"sh","version":"0"}}}' \
  '{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"search_components","arguments":{"query":"date picker"}}}' \
  | rust-ui --source https://github.com/Amit-1of1/Rust-UI- mcp
```

`npx @modelcontextprotocol/inspector rust-ui --source https://github.com/Amit-1of1/Rust-UI- mcp` opens a UI for the same thing.

## Stability

Experimental (new on 2026-10-15): tool names and arguments may still change. It speaks protocol revisions 2025-06-18, 2025-03-26 and 2024-11-05. `crates/cli/tests/mcp.rs` drives it over stdio the way a client does.
