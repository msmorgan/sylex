---
needs: []
---

# A Reading stores no capitalization

## Why

Every word and numeral leaf of a Reading stores which case was written (`SurfaceCase`, `Declared`
or `Initial`), and the grammar never reads it: a Lexeme that can take an initial capital is
admitted with one anywhere. Three things follow.

- A Reading built from a surface term has no source for the field: no `spelling` states a case
  and nothing derives one, so spelling cannot build the Reading that Realization needs.
- A text gets Readings that exist only by a capital in the wrong place. "When The Eighth Doctor
  enters, mill three cards." has two Derivations, and in the second "The" is the determiner
  *the* over "Eighth Doctor".
- Round-trip law 2 holds for a hand-built Reading only because its builder copies the case from
  the text it expects.

The capitals of Oracle text are positional. A word whose Lexeme is declared in lower case is
capitalized at the start of a unit or line, after a sentence's period, after a cost's colon,
after an em dash, a bullet or a table bar, as each item of a cost list (*{2}{B}, Sacrifice a
creature:*), at the start of a quoted ability, and in every position of a Type Line; it is lower
case everywhere else. The Open questions list the texts that do not fit.

## Design

- Realization derives case from the Reading's structure, as it derives punctuation
  (`docs/design/english.md` §7). Which positions open with a capital is the grammar's, stated on
  its Productions; the engine applies it and names no Category.
- Lexical analysis stays independent of the grammar: a Lexeme that takes an initial capital is
  analyzed in either case, and the parse admits the capitalized spelling only where the structure
  calls for one, as Adjacency Agreement admits *an* only before a vowel Onset. A Lexeme spelled
  exactly (a Proper Name, a symbol) keeps its declared spelling in every position.

## Open questions

- **A text whose capitals are not the derived ones.** Necratog prints "Exile the top creature
  card of your graveyard: this creature gets +2/+2 until end of turn." and Ogre Marauder "it
  gains "this creature can't be blocked" until end of turn", each lower case where every other
  unit has a capital. Under derivation such a text has no Reading. Whether an Editorial
  Correction reads it, and on what ground (`crates/english/corrections.txt` admits a data error
  against the printed card or ungrammatical English, never a template difference).
- **A quoted keyword ability is printed in both cases.** *"Cumulative upkeep {1}."* (Balduvian
  Shaman), *"Ward—Pay 2 life."* (Hexing Squelcher) and *"Cascade, cascade."* (Zhulodok, Void
  Gorger) open with a capital; *"bands with other legendary creatures."* (Adventurers'
  Guildhouse and its four siblings), *"bands with other creatures named Wolves of the Hunt."*
  (Master of the Hunt), *"enchant creature card in a graveyard"* (Animate Dead, Dance of the
  Dead) and *"enchant creature"* (Takklemaggot) do not. Nothing in
  their structure tells them apart, so here the case is the text's choice. Whether the grammar
  has two Productions for a quoted keyword ability, so that the tree says which was written as
  it does for two punctuation forms, or the lower-case ones are corrected.

## Done when

- The Open questions are answered in `docs/design/english.md`, and §7 says a Reading stores no
  capitalization beside its rule for punctuation.
- No leaf of a Reading carries a case: `LexicalValue` and `LexicalReading::Numeral` have no
  `capitalization` field.
- Both Round-trip Laws hold for every read unit with the case derived, and `cargo xtask english
  strata --check` passes. The landing lists every unit whose verdict or Readings change, with the
  Readings that needed a capital in the wrong place among them.
- A judgment pins that "When The Eighth Doctor enters, mill three cards." has no Reading in which
  "The" is a determiner.
- A Reading built by hand states no case and realizes with the text's capitals
  (`crates/english/tests/stratum_zero.rs`, `crates/english/tests/seed_proof.rs`).
- `cargo test` and `cargo clippy --tests -- -D warnings` pass for `sylex_lexicon`,
  `sylex_syntax`, `sylex_vocabulary`, `sylex_english` and `xtask`.

## Not in scope

- The Adjacency Agreement dimensions a leaf stores (*a*/*an*, *non*/*non-*, *'s*/*'*). The parse
  already checks them against the neighboring word.
- A numeral's notation and a name's Full or Shortened form: the text chooses those.

## Pointers

`crates/lexicon/src/types.rs` (`SurfaceCase`, `LexicalValue`, `SurfaceFeatures`),
`crates/lexicon/src/model.rs` (`Capitalization`), `crates/lexicon/src/analysis.rs`,
`crates/syntax/src/reading.rs` (`realize`, `join`), `crates/syntax/src/compile.rs`
(`ValueFacts`); `docs/design/english.md` §7; `crates/english/corrections.txt`.
