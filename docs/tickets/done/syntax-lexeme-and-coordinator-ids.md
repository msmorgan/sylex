---
needs: []
---

# Dense `Lexeme` and coordinator ids in `sylex_syntax`

## Why

`sylex_syntax` carries declared Lexemes everywhere as `LexemeId` (`dpsi::Ident`, 16 bytes, hashed
and ordered by string bytes), bloating `State` and `Summary` in the chart and paying string-order
`BTreeMap` and `BTreeSet` probes across lexeme, class, extension, `edges`, `homograph_forms` and
`Prediction` tables. Separately, `Production::link` puns a Linked Coordination's synthesized
serial-tail Category name as `Option<LexemeId>`, which flows into `Summary::coordinator`,
`Summary::link`, `State::headed_link` and `State::conjunct_coordinators` and forces `xtask`
(`english forests`) to probe both the lexicon and the category table on `Parse::node_coordinator`
to tell a Lexeme from a Linked Coordination.

## Design

- `CompiledGrammar` interns declared `LexemeId`s into a dense 32-bit `Lexeme` newtype
  (`NonZeroU32` niche so `Option<Lexeme>` is 4 bytes; Vintage exceeds 32k card names) assigned in
  `LexemeId` order (matching `Lexicon::lexemes`, so dense id order equals `LexemeId` order).
- Lexeme, class, extension, card-name, `Prediction::lexemes`, `edges` and `homograph_forms` tables,
  lexical tests and internal chart/forest state (`State`, `Summary`, `LeafFacts`, `Production`,
  `Extension`, `Elided`) use the dense `Lexeme` id, converting back to `LexemeId` at public and
  `Reading` boundaries.
- Linked Coordination identity is separated from `Lexeme` into its own typed id internally and
  exposed from `Parse::node_coordinator` as a typed coordinator enum distinguishing a Lexeme
  coordinator from a Linked Coordination, removing `xtask`'s lexicon/category probe.

## Done when

- On the full pool, `cargo xtask english forests` produces table-by-table identical content to
  before the change, and `cargo xtask english strata --check` writes an identical `strata.json`
  report.
- No code in `sylex_syntax` stores a Category name in a `Lexeme` or `LexemeId` field, and
  `crates/xtask/src/english/forests.rs` classifies coordinators directly from
  `Parse::node_coordinator` without probing the lexicon or category table.
- `cargo test` and `cargo clippy --tests -- -D warnings` pass for `sylex_syntax`, `sylex_english`
  and `xtask`.

## Not in scope

- Bind-time enum pre-resolution, typed child roles and `Production` / `normal_form` structural
  precomputation (`syntax-bind-time-types`).
- Single-copy chart deduplication, boundary-indexed chart tables and shared forest traversals
  (`syntax-chart-and-forest-dedup`).
- Changing the grammar (`english.fug`), admitted Readings or `Summary` Packing equivalence.

## Pointers

- `crates/syntax/src/compile.rs` (`CompiledGrammar`), `crates/syntax/src/prediction.rs`
  (`Prediction`).
- `crates/syntax/src/step.rs` (`State`, `Summary`, `LeafFacts`, `Production`, `Elided`),
  `crates/syntax/src/chart.rs` (`Parse::node_coordinator`).
- `crates/xtask/src/english/forests.rs` (`Coordinator`, `unit_rows`).
