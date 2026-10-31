### Security

- **cli:** `add`, `update` and the MCP server treat a `--source` registry and the lockfile as untrusted: item names must be module names and every file `<item>/<file name>`, so a remote `registry.json` or an edited `rust-ui-lock.json` can no longer write, read or delete outside the item folders. Symbolic links are never copied from a source or written through in the app. `git clone` gets `--` before the URL. (#65)
- **deps:** `cargo deny check` (advisories, licences, git sources, wildcards) in `scripts/check.sh` when installed, from a new `deny.toml`; `Cargo.lock` moves to fixed `rustls`, `rustls-webpki`, `tokio`, `pulldown-cmark` and the un-yanked `clipboard-win`. (#65)
