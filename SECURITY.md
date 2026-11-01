# Security policy

## Reporting a vulnerability

Please **don't open a public issue**. Report it privately through GitHub: the repository's **Security** tab, then **Report a vulnerability** ([direct link](https://github.com/Amit-1of1/Rust-UI-/security/advisories/new)). Include what's affected (crate, component or `atoll` command, and version or commit), how to reproduce it, and the impact you see.

What happens next ([service levels](docs/MAINTAINING.md#service-levels)):

1. A first response within 3 working days.
2. A fix or mitigation within 14 days, released as a patch and backported to the previous minor release while it's supported ([docs/VERSIONING.md](docs/VERSIONING.md#support-while-0x)).
3. A published GitHub security advisory crediting you, unless you'd rather stay anonymous.

## Supported versions

Nothing has been released yet; until `v0.1.0`, fixes go to `main`. After it: the latest minor release, and the previous minor for three months after a new one ships.

## Scope

In scope: the `atoll-ui`, `atoll-motion`, `atoll-form-derive` and `atoll-cli` crates, the `atoll-primitives` fork, and the registry and MCP server (`atoll add`, `atoll update`, `atoll mcp`), which treat a `--source` registry and the lockfile as untrusted input. Out of scope: the example apps' demo data, and vulnerabilities in upstream Dioxus or Blitz that our forks don't change (report those upstream; tell us too if Atoll needs a workaround).

`cargo deny` checks advisories and licences of every dependency in `scripts/check.sh` and CI.
