---
needs: [english-forest-counts-and-forks, english-forest-forks, english-parse-corpus-load]
---

# `cargo xtask english forestwalk`: survey a unit's forest

## Why

`english parse` shows a unit's ambiguity as its best `--limit` Readings, each a full constituent
table, so a unit with thousands of Readings (*Draugr Necromancer*, 3,312) cannot be read
whole, and where its Readings differ must be found by comparing trees by eye.
`english-forest-counts-and-forks` computes, without enumeration, what every Derivation shares,
and `english-forest-forks` where they part (`sylex_syntax::ForkForest::forks`). This command
prints that, short enough to read.

## Design

- One stateless call per unit: no session, no state between calls. The unit is given as
  `english parse` gives it: `--card NAME` fetches that card's own units from the supported
  corpus, and text, alone or with `--as-card NAME`, is read as stated with nothing fetched
  (judgment entry syntax included: `<card=…>`, Category prefix).
- With `--card NAME`, exactly one of `--line N` (1-based, a rules-text line) and `--type-line`
  picks the unit, as `english parse` labels it (`FACE line N`, `FACE type line`). `NAME` is a
  face's name, or the whole card's when the card has one face. Card name and `Unit::index` alone
  would not do: a face's type line and its first rules-text line both have index 0, and a
  double-faced card's faces share its card name.
- Each of these is an error that names the way out: neither flag or both (it lists the face's
  units as `parse` labels them); a line the face lacks; a `NAME` matching faces of more than one
  name (it lists the face names); either flag with stated text.
- A face name two cards share (*Lightning Bolt*, also a face of *Emeritus of Conflict //
  Lightning Bolt*) picks one unit when the faces' picked units have the same text, since the
  survey reads only the text and the face's name; when they differ it is an error that shows
  both.
- Output: the settled spine (the settled nodes from the root, single-child pass-through chains
  folded into one line), then every fork in the order `ForkForest::forks` gives, named as it
  names it (kind, leaf surface and occurrence, aliases, whether conditional), with its options
  and each option's exact count; tens of lines for a unit of thousands of Readings.
- A word, a fork's own or a governor's, is printed as `"surface"#N`, its surface and 1-based
  occurrence (`ForkWord`). The occurrence is always printed, for a surface that occurs once too:
  it is the spelling `english-forestwalk-narrowing`'s selectors take.
- An option may show the best Ranking Key among its Readings as information, labeled so that it
  does not read as a recommendation (`docs/decisions/english.md` §5). Options are never ordered
  or marked by it.

## Done when

- `cargo xtask english forestwalk` prints the survey for a unit of a card fetched with `--card`
  and for stated text, with `--as-card` or as a judgment entry. It fetches a card only for
  `--card`, and computes a unit's Card Scope from its face's name (`corpus::own_names`,
  `Judgment::scope`).
- On *Draugr Necromancer*'s second line, *Maralen, Fae Ascendant*'s third and *Remove
  Enchantments*' only line the survey is at most a few tens of lines, and the counts it prints
  agree with `english parse`'s `Packed Forest derivations`.
- A test pins the output shape on a real unit with a single Reading (no forks) and on one with
  a dependency fork.
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
