# Pro: how the paid tier works

The kit is free and open source. **Pro** is a set of extra items (premium blocks, app
templates, motion components) sold for a one-time price and installed with the same tools.
Why this model, the prices and the risks are in the decision
[2026-10-30-monetisation.md](../decisions/2026-10-30-monetisation.md) and the plan in the
project files (`rust-ui/monetisation/PLAN.md`). Nothing is on sale yet.

## Pro v1

Built on 2026-10-31: 13 marketing blocks (header, two heroes, logo cloud, two feature
sections, stats, two pricing sections, testimonials, FAQ, call to action, footer), two motion
components (`reveal`, `number_ticker`) and a SaaS starter app. The public list with screenshots
is [CATALOG.md](CATALOG.md) ([catalog.json](catalog.json) for tools and the docs site). The
sources are kept **outside this repository**, in a folder laid out like it (the "Pro source
folder"); `scripts/pro-repo.sh --overlay` puts them into the Pro repository. Its own
`PRO.md` lists the items and the checks (`scripts/pro-e2e.sh`, `.github/workflows/pro.yml`).

## How it fits together

| Piece | Where | Status |
| --- | --- | --- |
| Free kit | this repository (public once the owner makes it so) | done |
| Pro items | a **private** copy of this repository that merges `main` and adds Pro items | `scripts/pro-repo.sh` builds it locally |
| Marking an item Pro | a `LICENSE-PRO.md` in the item's folder (copy [LICENSE-PRO.md](LICENSE-PRO.md)) | done: the registry writes `"tier": "pro"` |
| Telling the buyer | `atoll list` shows `(pro)`; `atoll add` and the MCP server's `add_component` print the licence reminder | done |
| Pricing page | docs site, `#pricing` (`apps/docs/src/pricing.rs`) | done; buttons off until a checkout URL is set |
| Checkout and access | Polar product with the "GitHub repository access" benefit on the Pro repo | owner's account |

The buyer's side:

```sh
# after paying, their GitHub account is invited to the Pro repository
export ATOLL_SOURCE=https://github.com/Amit-1of1/atoll-pro
atoll list                      # free and Pro items, Pro ones marked (pro)
atoll add pricing_02            # copies the block, its components and its LICENSE-PRO.md
dx components add pricing_02 --git https://github.com/Amit-1of1/atoll-pro   # also works
```

Both clone with the buyer's own git credentials, so no licence server or API key is needed.
A refund removes the invitation, and with it the access to new versions.

## Rules for Pro items

- They follow every rule in [GUARDRAILS.md](../GUARDRAILS.md), as the decision file explains:
  kit components only in blocks, keyboard access, reduced motion, theme tokens, the aria and axe
  checks, a `THIRD_PARTY.md` row for anything copied.
- They live **only** in the Pro repository. `crates/cli/tests/cli.rs`
  (`the_public_registry_has_no_pro_items`) fails if one lands here; the Pro repository skips
  that test through its `.atoll-pro` marker.
- Each has `LICENSE-PRO.md` in its folder, so the licence travels with the copied code.

## Running the Pro repository

1. `scripts/pro-repo.sh ../atoll-pro --overlay <Pro source folder>` (add `--demo` instead of
   `--overlay` to watch a block become Pro and get installed into a throwaway app).
2. Create a private repository on GitHub, then
   `git -C ../atoll-pro remote add origin <its URL> && git -C ../atoll-pro push -u origin main`.
3. Build Pro items there with `atoll new` or by adding block folders, as in this repo, then
   copy `docs/pro/LICENSE-PRO.md` into each one and run `atoll registry`. Copy new Pro
   files back into the Pro source folder too, so `--overlay` can always rebuild the repository.
   Add each new item to [catalog.json](catalog.json) with a screenshot.
4. To pick up free-kit changes: `git -C ../atoll-pro pull upstream main`, then
   `atoll registry` and `scripts/check.sh`.

## Before launch (needs the owner's accounts)

1. Pick the product name (Rust/UI at rust-ui.com is another, older kit) and set `PRODUCT` in
   `apps/docs/src/pricing.rs`.
2. Make this repository public and turn on Pages (docs/PUBLISHING.md).
3. Create the private Pro repository and push what `scripts/pro-repo.sh` built.
4. Create the Polar products (Pro Personal, Pro Team) with the GitHub repository access
   benefit, and paste their checkout links into `CHECKOUT_URL_PERSONAL` and
   `CHECKOUT_URL_TEAM` in `apps/docs/src/pricing.rs`.
5. Set up GitHub Sponsors and add `.github/FUNDING.yml`.
6. Have a lawyer review [LICENSE-PRO.md](LICENSE-PRO.md).
