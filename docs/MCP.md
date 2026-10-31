# MCP server

`atoll mcp` serves the registry to AI coding tools over the [Model Context Protocol](https://modelcontextprotocol.io) (stdio). With it, an agent working in your Dioxus app can find a component, read its props and example, and copy it into the app without you pasting docs or running commands.

It is the same CLI as the rest of `atoll`, so install that first:

```sh
cargo install --git https://github.com/Amit-1of1/Rust-UI- atoll-cli   # installs `atoll`
```

## What the agent gets

| Tool | Does | Same as |
| --- | --- | --- |
| `list_components` | Every component and block, with a one-line description and its dependencies. `kind`: `component`, `block` or `all`. | `atoll list` |
| `search_components` | Components matching every word of `query`, names first, then titles and docs. `limit` defaults to 10. | |
| `get_component` | One item's page: install and import lines, docs, prop signatures and an example. `include_source: true` adds every file. | `atoll docs <name>` |
| `add_component` | Copies `names` and everything they depend on into `dest` (default `src/components`) and blocks into `blocks_dest` (default `src/blocks`). Keeps files you edited unless `force`. Replies with the `pub mod` lines to add. | `atoll add` |
| `init_theme` | Copies the theme stylesheet to `dest` (default `assets/theme.css`). | `atoll init` |
| `explain_errors` | Runs `cargo check` in the project (extra `cargo_args` allowed), or reads `cargo_output`, and explains errors from kit components in plain words: the missing or misspelt prop and its docs, the enum variants that exist, the component to add. | `atoll explain` |
| `get_contract` | A component's keyboard contract: what each key does, the WAI-ARIA pattern, native support. Every row is a test the kit runs. Without `name`, the components that have one. | `atoll contract <name>` |
| `generate_theme` | An accent preset from one `brand` hex colour, readable (WCAG AA) in light and dark, with the contrast it reaches. | `atoll theme --brand` |
| `list_templates` | The example apps a new app can start from, with the command for each. | `atoll create --template` |

The pages are also resources, for clients that let you attach them: `atoll://llms.txt` (the index) and `atoll://docs/<name>`.

`add_component` and `init_theme` write only inside the project, and `explain_errors` runs `cargo check` only there: paths are relative to it, and absolute paths or `..` are refused. The project is the folder the client starts the server in, or `--project <dir>`.

## Where components come from

`--source` works as for every other command: a git URL is shallow-cloned into `~/.cache/atoll` and updated each time the server starts (it needs `git`); a local checkout is read as it is, which is how you pin a version. `ATOLL_SOURCE` works too.

## Claude Code

From your app's folder:

```sh
claude mcp add atoll -- atoll --source https://github.com/Amit-1of1/Rust-UI- mcp
```

Add `--scope project` to write it to `.mcp.json` so everyone on the project gets it. The file looks like this:

```json
{
  "mcpServers": {
    "atoll": {
      "command": "atoll",
      "args": ["--source", "https://github.com/Amit-1of1/Rust-UI-", "mcp"]
    }
  }
}
```

`claude mcp list` should show `atoll: ... - ✓ Connected`.

## Cursor

The same `mcpServers` block as above, in `.cursor/mcp.json` in the project (or `~/.cursor/mcp.json` for every project).

## VS Code (Copilot agent mode)

`.vscode/mcp.json`:

```json
{
  "servers": {
    "atoll": {
      "type": "stdio",
      "command": "atoll",
      "args": ["--source", "https://github.com/Amit-1of1/Rust-UI-", "mcp"]
    }
  }
}
```

## Codex CLI

`~/.codex/config.toml`:

```toml
[mcp_servers.atoll]
command = "atoll"
args = ["--source", "https://github.com/Amit-1of1/Rust-UI-", "mcp"]
```

## Claude Desktop and other clients without a project folder

Clients that don't start servers in your project need `--project` with an absolute path, otherwise `add_component` writes into whatever folder the client runs in. In `claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "atoll": {
      "command": "atoll",
      "args": ["--source", "https://github.com/Amit-1of1/Rust-UI-", "mcp", "--project", "/path/to/your/app"]
    }
  }
}
```

If the client can't find `atoll`, give `command` the full path (`~/.cargo/bin/atoll`, expanded).

## Agent skill and `atoll info`

MCP gives the agent tools; the skill tells it how to use the kit well. `skills/atoll/SKILL.md` is an [Agent Skill](https://agentskills.io): when to use the kit, to read the project with `atoll info --json` first, which component fits which need, the accessibility and motion rules, and to update with `atoll update` instead of `add --force`. Install it into your app:

```sh
atoll --source https://github.com/Amit-1of1/Rust-UI- skill            # writes .claude/skills/atoll/SKILL.md
atoll --source https://github.com/Amit-1of1/Rust-UI- skill --dest .agents/skills/atoll/SKILL.md
```

`atoll info` (`--json` for agents) reads only your app: the `dioxus` requirement in Cargo.toml, `assets/theme.css` and its accent presets, every item in `atoll-lock.json` with the files you edited, files still holding update conflict markers, and whether the skill and an MCP config are present. `crates/cli/tests/info.rs` covers it, and checks that every component the skill names exists in the registry.

## Trying it by hand

The server reads one JSON-RPC message per line and answers on stdout:

```sh
printf '%s\n' \
  '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"sh","version":"0"}}}' \
  '{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"search_components","arguments":{"query":"date picker"}}}' \
  | atoll --source https://github.com/Amit-1of1/Rust-UI- mcp
```

`npx @modelcontextprotocol/inspector atoll --source https://github.com/Amit-1of1/Rust-UI- mcp` opens a UI for the same thing.

## Stability

Experimental (new on 2026-10-15): tool names and arguments may still change. It speaks protocol revisions 2025-06-18, 2025-03-26 and 2024-11-05. `crates/cli/tests/mcp.rs` drives it over stdio the way a client does.
