# Design tokens for design tools

`atoll tokens` writes the theme's tokens as [W3C Design Tokens](https://www.designtokens.org/) JSON (DTCG format 2025.10), one file per colour mode:

```sh
atoll tokens                       # tokens/light.tokens.json and tokens/dark.tokens.json
atoll tokens --accent blue         # with an accent preset applied
atoll tokens --theme brand.css     # with your own theme or preset loaded after the kit's
atoll tokens --out design/tokens   # somewhere else
```

Import the two files as the "Light" and "Dark" modes of one collection in Figma (Variables, Import), Tokens Studio, Penpot, or feed them to Style Dictionary. Designers then work from the values the code uses instead of a hand-copied palette, which is the usual way design and code drift apart.

What it writes:

- Colours as hex (`#rrggbb`, or `#rrggbbaa` with transparency), each `light-dark()` pair split into its mode.
- A token that is only `var(--other)` stays a reference (`"{other}"`), so the design tool keeps the link: change the base colour and the alias follows.
- Sizes (`px`, `rem`) as `dimension`, times as `duration` in ms, `cubic-bezier()` as `cubicBezier`, font stacks as `fontFamily`, plain numbers as `number`.
- Tokens with no DTCG type (shadows, gradients, spring settings, the `--light`/`--dark` toggles) are left out and listed when the command runs.

`theme.css` stays the source of truth. Re-run the command after changing it; don't edit the JSON.
