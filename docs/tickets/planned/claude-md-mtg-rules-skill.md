---
needs: []
---

# Name the mtg-rules skill in CLAUDE.md for rules-modeling work

## Why

`CLAUDE.md` routes all CR work to `cargo xtask cite`, which only checks that cited rules exist and
pins their text. It never mentions the mtg-rules skill, which supplies rule text, card data,
rulings and maps of the rules a subsystem spans, so agents designing or auditing rules encodings
work without those maps.

## Done when

- The "CR citations" section of `CLAUDE.md` (which `AGENTS.md` links to) gains one short line
  saying: `cargo xtask cite` checks cited rules and pins their text; the mtg-rules skill supplies
  rule text, card data, rulings and subsystem maps, and before designing or auditing anything that
  encodes the rules, read the matching map section (`scripts/ref <map> <section>`); cite rules,
  not maps.
- The edit follows the `writing-for-agents` skill.

## Pointers

- `CLAUDE.md` (CR citations)
- The installed mtg-rules plugin: `skills/mtg-rules/scripts/ref` and `skills/mtg-rules/references/`.
