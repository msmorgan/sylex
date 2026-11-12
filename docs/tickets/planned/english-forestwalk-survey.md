---
needs: [english-forest-counts-and-forks, english-forest-forks, english-parse-corpus-load]
---

# `cargo xtask english forestwalk`: survey a unit's forest

## Why

`english parse` shows a unit's ambiguity as its best `--limit` Readings, each a full constituent
table, so a unit with thousands of Readings (*Draugr Necromancer*, 3,312) cannot be read
whole, and where its Readings differ must be found by comparing trees by eye.
`english-forest-counts-and-forks` computes, without enumeration, what every Derivation shares,
and `english-forest-forks` where they part. This command prints that, short enough to read.

## Design

- One stateless call per unit: no session, no state between calls. The unit is given as
  `english parse` gives it: `--card NAME` fetches that card's own units from the supported
  corpus, and text, alone or with `--as-card NAME`, is read as stated with nothing fetched
  (judgment entry syntax included: `<card=…>`, Category prefix).
- Output: the settled spine (the settled nodes from the root, single-child pass-through chains
  folded into one line), then every unresolved fork with its anchor `wN`, its options and each
  option's exact count; tens of lines for a unit of thousands of Readings.
- An option may show the best Ranking Key among its Readings as information, labeled so that it
  does not read as a recommendation (`docs/decisions/english.md` §5). Options are never ordered
  or marked by it.

## Open questions

- How one unit of a fetched card is picked. `english parse --card NAME` reads every unit of
  every face so named and labels each `FACE type line` or `FACE line N` (1-based). A face's type
  line and its first rules-text line both have index 0 (`Unit::index`, `Face::units` in
  `corpus.rs`), and a double-faced card's faces share its card name, so card and index alone
  are ambiguous. Options: face name plus a 1-based line number with the type line addressed
  apart, as `parse` labels them; the kind and index as `Unit` stores them; or the unit's text.

## Done when

- `cargo xtask english forestwalk` prints the survey for a unit of a card fetched with `--card`
  and for stated text, with `--as-card` or as a judgment entry. It fetches a card only for
  `--card`, and computes a unit's Card Scope from its face's name (`corpus::own_names`,
  `Judgment::scope`).
- On *Draugr Necromancer*'s second line, *Maralen, Fae Ascendant*'s third and *Remove
  Enchantments*' only line the survey is at most a few tens of lines, and the counts it prints
  agree with `english parse`'s `Packed Forest derivations`.
- A test pins the output shape on a real unit with a single Reading (no forks) and on one with
  an attachment fork, and checks that every fork's option counts sum as
  `english-forest-forks` states.
- `cargo test -p xtask` and `cargo clippy -p xtask --tests -- -D warnings` pass.

## Not in scope

- Picking or excluding options: `english-forestwalk-narrowing`.
- Any choice among options; the command reports the forest and decides nothing (`english.md` §4).

## Pointers

- `crates/xtask/src/english.rs` (`parse`, `fetched_items`, `stated_items`, `dump_reading`,
  `format_key`).
- `crates/english/src/corpus.rs` (`Unit`, `Face::units`), `crates/english/src/judgment.rs`
  (`parse_file`, `Judgment::scope`), `crates/english/judgments/README.md`.
- `docs/decisions/english.md` §4, §5.
