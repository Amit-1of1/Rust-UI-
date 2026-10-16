# Working in parallel without merge conflicts

Several Claude sessions work on this repo at once, each on its own branch and PR, and the
owner merges them. This is the protocol they follow so a merge never leaves the other PRs
conflicted for longer than the sync bot takes. The checklist form is in
[GUARDRAILS.md, Parallel sessions](GUARDRAILS.md#parallel-sessions); this file explains who
does what and why.

## Why PRs still conflicted after PR #41

PR #41 made the usual hot spots (changelog, module lists, generated files) safe to merge
*locally* with `scripts/sync-main.sh`. GitHub kept showing conflicts for two reasons
(checked on 2026-10-16 with `scripts/pr-conflicts.sh` and by replaying every merge since #41):

1. **Nobody synced between merges.** Generated files are committed, so every merge to main
   makes each open PR that also regenerated them conflict on GitHub until its session runs
   sync-main. Sessions only sync when woken, and the owner merged #46, #47 and #44 within one
   minute. After #48 merged, #49 conflicted in `llms-full.txt`, `llms/dialog.md` and
   `llms/alert_dialog.md`: all generated, all fixed by one sync-main run nobody started.
2. **Two hand-edited spots were still shared:** the one-line `members = [...]` list in the root
   `Cargo.toml` (#45 against #46, both adding a crate) and the dependency lists in each
   `Cargo.toml` (#49 and #50 both add two dependencies at the same place in
   `apps/native/Cargo.toml`).

## What fixes it

| Cause | Fix |
| --- | --- |
| No sync between merges | `.github/workflows/sync-prs.yml`: on every push to main it runs `scripts/sync-main.sh` on each open PR branch from this repo, pushes the merge and starts CI on it. A PR that still conflicts gets one comment listing the files. |
| Workspace member list | `members = ["crates/*", "apps/*"]`: adding a crate or app no longer edits the root `Cargo.toml`. |
| Dependency lists | `scripts/sync-main.sh` keeps both sides' added lines in a conflicted `Cargo.toml` when cargo still reads the result; otherwise it leaves the file for the session. |
| Not knowing what collides | `scripts/pr-conflicts.sh` lists, for every open branch, what conflicts with main and with each other branch, marking generated files. |

## Who does what

**The owner (merges):**

1. Merge one PR at a time.
2. After a merge, wait for the "Sync open PRs with main" run to finish (Actions tab) before
   merging the next; it takes a few minutes. A PR that shows conflicts after that run has a
   comment from it, and its session is on it.
3. Order: fork pins and dependency bumps, then shared infrastructure (scripts, CI, build), then
   features, smallest first. When `scripts/pr-conflicts.sh` shows two PRs conflicting with each
   other in non-generated files, merge the one that was ready first.

**The coordinator (the project's main session):**

1. Every thread brief carries the rules in "Rules for every brief" below.
2. Before starting a thread whose work will touch a shared file (the list below), check whether
   an open PR already changes it (`scripts/pr-conflicts.sh` or the PR's file list). If one does,
   say so in the brief: the new thread waits for that PR, or keeps its change to its own lines.
3. When the owner merges, tell the threads whose PRs the sync bot commented on to sync.

**Each thread session (owns exactly one PR):**

1. Branch from the latest `origin/main`; never from another PR's branch.
2. Add files instead of appending to shared ones: `changes/<topic>.md` for changelog entries,
   `tests/components/<name>.rs`, `docs/notes/<topic>.md`, a new crate or app folder (the
   workspace globs pick it up).
3. Before every push run `scripts/sync-main.sh`, then `scripts/check.sh`. sync-main also merges
   your own branch from origin, since the sync bot may have pushed a merge to it. Never rebase,
   amend or force-push a branch with an open PR: it would drop the bot's merge.
4. When the PR gets a comment from the sync bot, a push from it, or a conflict notice: run
   `scripts/sync-main.sh`, resolve what it lists, `scripts/sync-main.sh --continue`,
   `scripts/check.sh`, push. That is the session's job, not the owner's.
5. Never merge PRs, and never push to another session's branch.

## Shared files that are still hand-edited

Change only your own lines and put new lines next to related ones, not at the end:
the gallery's sections in `apps/gallery/src/main.rs`, `scripts/check.sh`, `scripts/e2e.sh`,
`.github/workflows/*`, `tests/class.rs`, `apps/*/GAPS.md`, `docs/STABILITY.md`, the
`[workspace.dependencies]` and `[patch.crates-io]` tables in `Cargo.toml`, and each crate's
dependency list. A change to `[patch.crates-io]` (a fork pin) rewrites `Cargo.lock` and every
`component.json`; ship it alone and merge it first.

## Rules for every brief

The coordinator pastes these into project memory and every thread brief:

1. Branch from the latest `origin/main`; one PR per thread.
2. Add files, don't append: `changes/<topic>.md`, `tests/components/<name>.rs`, `docs/notes/`.
3. Run `scripts/sync-main.sh` then `scripts/check.sh` before every push; never rebase or
   force-push.
4. A sync-bot comment, sync-bot push or conflict notice on your PR means: sync-main, resolve,
   check, push, now.
5. Never hand-edit or hand-merge generated files (`registry.json`, `component.json`, `llms*`,
   `apps/docs/src/generated.rs`, `Cargo.lock`, snapshots).
6. Name any shared file you will change (list in docs/PROTOCOL.md) in your first reply, so
   overlapping work is caught before it conflicts.

## Limits

- The sync bot only handles PRs from branches in this repo, and runs the branch's own build
  code with a token that can push; PRs from forks are skipped.
- It can't push a merge that changes a file in `.github/workflows/` (GitHub doesn't let the
  workflow token do that); those PRs get a comment and their session syncs by hand.
- GitHub does not run CI on pushes made by the workflow token, so the bot starts `CI` and
  `Browser checks` on the branch itself.
