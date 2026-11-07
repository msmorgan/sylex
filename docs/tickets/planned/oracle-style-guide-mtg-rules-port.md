---
needs: []
---

# Port the corrected Oracle style guide from the mtg-rules plugin

**Why.** The mtg-rules plugin carries a corrected copy of this repository's style guide,
`~/Projects/agent-plugins/mtg-rules/skills/mtg-rules/references/oracle-style.md`. Its corrections
fix claims in `docs/oracle-style-guide.md` that contradict the rules or the corpus: `dies` limited
to creatures [CR#700.4], numerals beyond ten, comparative forms with a variable, the scope of
`this way`, what `another`/`other` exclude, `in addition to its other types` versus `still a
land` [CR#205.1b], modal choices made by another player [CR#700.2e], `tapped and attacking` for
every token put into combat [CR#110.5b,508.1f], the named-token appositive [CR#111.9], counters
tested without `on it`, and continuous changes called one-shot [CR#610.1,611.2a].

**Done when.**
- `diff -u docs/oracle-style-guide.md <plugin copy>` shows only the provenance hunk: this guide
  keeps its own provenance line and does not add the copy's "(effective 2026-09-25)".
- The port follows the guide's front matter: a copy-editing register, with no parser, engine or
  corpus-count talk. Existing `[CR#…]` citations are corrected (the modal-choice passage's
  `[CR#700.2]` becomes `[CR#700.2e]`), and no new ones are added.
- The `enters` passage states the short form as a preference; *Brainstealer Dragon*'s "enters the
  battlefield under your control" stays as printed.
- `cargo xtask cite check --list-noncompliant` is empty, `cargo xtask cite check` reports 0 stale,
  and newly cited rules are blessed.

**Not in scope.** Making the English grammar accept both numeral forms and both token-order
forms, and keep each card's form in round trips.

**Pointers.** `docs/oracle-style-guide.md`, the plugin copy above.
