### Added

- **cli:** Pro items: an item whose folder holds a `LICENSE-PRO.md` is `"tier": "pro"` in `registry.json`; `atoll list` marks it `(pro)`, and `atoll add` (and the MCP server's `add_component`, whose `list_components` now reports `tier`) prints a licence reminder after copying one. The public registry has no Pro items. Experimental. ([docs/pro/README.md](docs/pro/README.md)) (#52)
- **docs:** a Pricing page (`#pricing`) on the docs site for the free kit and the planned Pro pack; its buy buttons stay disabled until a checkout URL is set. (#52)
- **scripts:** `scripts/pro-repo.sh <dir>` builds a local copy of the private Pro repository without pushing anything; `--demo` marks a block Pro and installs it into a throwaway app. (#52)
