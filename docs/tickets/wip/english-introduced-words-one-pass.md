---
needs: []
---

# Introduced words are read in one pass and discharged across the face

## Why

Three kinds of word are read from a face's own text, and each is found a different way.
An Introduced Name is read in one parse of its unit: each Constituent's summary records the names
it introduces and names again, and the root compares them (`crates/syntax/src/names.rs`). Anchor
Words are found by a first parse of the face's lines with every bulleted label in scope, then
every line is parsed again (`crates/english/src/anchors.rs`). Vote Options take up to three
parses of a line: one with every word in it as a candidate, one to find a mention, and the real
one (`crates/english/src/options.rs`). The two finding parses exist because a candidate must be
in the Card Scope before a parse can read it.

## Design

- A position that introduces words (the Anchor Choice of an entry choice, the Option Choice of a
  vote, the introduction of a name) reads any word standing there, with no candidate put in scope
  beforehand. The Reading that reads it introduces those words.
- A face's lines are parsed once each, in order. The words the lines so far introduce are
  available to the lines after them.
- A later use discharges an introduced word: a bulleted label for an Anchor Word, a mention for a
  Vote Option, a second naming for an Introduced Name. A Reading that introduces Anchor Words or
  Vote Options stands only if the face discharges every word it introduces; otherwise that
  Reading is refused and the unit's other Readings stand (*Roots of Life*'s "choose Island or
  Swamp" keeps its subtype Reading and has no Anchor Choice).
- Names, Anchor Words and Vote Options use this one mechanism. `anchors.rs` and `options.rs`
  make no finding parse.
- A stated input still gives its words with `<anchor=…>` and `<vote=…>`.

## Done when

- No line is parsed more than once to read a face, and the three kinds of word share one
  introduce-and-discharge mechanism.
- The 14 anchor-word cards, the 26 word-vote faces and every unit with an Introduced Name keep
  their Readings, and *Roots of Life*, *Council Guardian* and the flavor-labeled modal bullets
  (*Cult of Skaro*, *Astarion, the Decadent*) keep theirs.
- A name used before its introduction in one unit still reads (*Rinoa Heartilly*), and the
  landing says what a word used on an earlier line than its introduction does, with the cards
  that show it or that none does.
- Parse and admission agree on every unit (`docs/design/english.md` §7), the census shows no
  verdict change, and `cargo xtask english strata --check`, `cargo test` and
  `cargo clippy --tests -- -D warnings` pass for `sylex_lexicon`, `sylex_syntax`,
  `sylex_english` and `xtask`.

## Pointers

- `crates/syntax/src/names.rs`, `crates/english/src/anchors.rs`, `crates/english/src/options.rs`,
  `crates/english/src/corpus.rs` (`Face::scope`, `read_scope`), `crates/lexicon/src/scope.rs`.
- `docs/design/lexicon.md` §2, §6; `docs/design/english.md` §7.
