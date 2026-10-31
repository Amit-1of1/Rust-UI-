# Change fragments

Each pull request describes its changes in its **own file here** instead of editing
`CHANGELOG.md` or `docs/MIGRATION.md`. Two branches that both edited the top of those files
always conflicted; two branches that each add a file never do.

## Adding one

1. Create `changes/<topic>.md`, named after what the branch does (`time-picker.md`,
   `parallel-sessions.md`). Use only these headings, in any order:
   `### Breaking`, `### Added`, `### Changed`, `### Deprecated`, `### Removed`, `### Fixed`,
   `### Security`. Write the entries exactly as they would appear in the changelog:

   ```markdown
   ### Added

   - **ui:** `TimePicker` with 12/24-hour input (experimental). (#39)
   ```

2. For a breaking change, also create `changes/<topic>.migration.md` with one or more
   `### <what changed> (PR #N)` sections in the MIGRATION.md format (Who, Before, After,
   What to do).

3. When the PR is open, put its number in the entries (`(#39)`). Only your own file changes,
   so this commit can't conflict either.

`atoll changelog --check` (run by `scripts/check.sh`) validates the fragments.

## Releasing

On `main`, before tagging: `cargo run -p atoll-cli -- changelog`. It adds every fragment's
entries to the top of the matching "Unreleased" sections (newest PR first), adds the
migration notes to the top of MIGRATION.md's "Unreleased", and deletes the fragments. Commit
that as its own PR. It refuses to run while a fragment has no PR number.
