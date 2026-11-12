# sylex_syntax

The generic syntax engine: it binds a grammar data file to a lexical
environment, parses text into a packed forest with an Earley chart, ranks and
enumerates Readings, and realizes them back to text. A grammar is data, loaded
at runtime; the engine has no proc macros and no generated code. It knows no
English and no Oracle: the Oracle English grammar is
[`sylex_english`](../english/README.md)'s.

The design and its reasons are in
[`docs/decisions/english.md`](../../docs/decisions/english.md) (the crate
split in §1, the Ranking Key in §5, the parser contract in §7, Readings in §8)
and [`docs/decisions/lexicon.md`](../../docs/decisions/lexicon.md). Terms
follow [`docs/contexts/oracle-english/CONTEXT.md`](../../docs/contexts/oracle-english/CONTEXT.md).
Mechanism is documented in rustdoc on the types:

```
cargo doc -p sylex_syntax --no-deps --document-private-items --open
```

## Data flow

1. **Load.** `Grammar::from_fug_str` reads a grammar
   of Categories, features, Productions and Schemas (`grammar.rs`).
2. **Bind.** `CompiledGrammar::bind(grammar, lexicon, frames)` binds it to a
   `sylex_lexicon::Lexicon` and a `FrameCatalog` of Subcategorization Frames
   (`frames.rs`), validates every name, and compiles the Schemas into
   Productions (`compile.rs`). Non-fatal findings are in
   `CompiledGrammar::report()`.
3. **Parse.** `parse(lexicon, text)`, `parse_analyzed(analyzed)` or
   `parse_category(analyzed, category)` runs the chart over the lexical
   analysis (`chart.rs`, `step.rs`, `prediction.rs`) and returns a `Parse`:
   the `PackedForest`, its `Metrics` and the vocabulary gaps.
4. **Rank.** `Parse::best_readings(k)`, `best_ranked(k)` and `readings()`
   lazily enumerate Readings by Ranking Key; `cheapest_tier()` and
   `derivation_count()` count without building trees (`reading.rs`).
5. **Check and write back.** `admit` re-checks a Reading through the chart's
   own step function, `realize` writes it out, `constituents` locates its
   Constituents in that text, and `normal_form` applies the approved
   canonicalizations (`normal_form.rs`). `Parse::verify_realization` and
   `Parse::contains` check both round-trip laws over the forest without
   enumerating trees (`laws.rs`).

## Schemas

Authors write the active Subcategorization Frames once and declare Schemas;
binding compiles each Schema into the Productions it implies, so no voice,
Gap or coordination shape is written by hand. The kinds are the variants of
`grammar::SchemaDecl`:

- `Valence`: a phrase per frame of every head lexeme of a lexical Category,
  with its passives, Gap variants, Complement Clusters and remnants.
- `Adjunct`: unselected attachment to a Host Category, with same-head
  complement priority derived from the frames.
- `Coordination`: binary, serial and polysyndetic coordination of a
  Category, with Right Node Raising.
- `Premodification`: an attributive head kept as a Modifier.
- `Parameters`: the Catalog's ordered parameter patterns around a head.

Everything else is an authored Production. The child names the engine acts
on are the contract documented on `grammar::ProductionDecl`.

## Inspecting

- `cargo xtask english parse "<text>" … [--limit N]` prints the cheapest
  Readings of each input under the English grammar, with their Ranking Keys
  and Constituents; `--as-card NAME` reads the text as if on that card, and
  `--card NAME` alone reads that card's own text.
- `Parse::metrics()` gives chart items, forest nodes, families and leaves.
- `PackedForest`'s accessors (`roots`, `node_span`, `node_items`,
  `item_families`, `leaf_span`), `Parse::count_node` and the `walk` module
  read the forest from outside; `sylex_english`'s census is built on them.
- The `tree-ranking-key` feature, enabled for this crate's tests, adds
  `CompiledGrammar::ranking_key`, which computes a Reading's key from its tree
  to check the forest's.

## Working rules

- **The engine names no grammar term.** Outside the lexicon feature-bundle
  mapping table (`PROJECTED` in `compile.rs`, `projected_values` and
  `leaf_facts` in `step.rs`), no feature, value or Category name appears in
  the engine; anything English belongs in `sylex_english`'s grammar data.
- **No defaults.** A derivation the grammar does not declare is not built, or
  binding fails; the engine never supplies a grammar default.
- **Canonicalizations are listed.** Every canonicalization the chart builds is
  listed in the `normal_form` module doc, marked where `normal_form`
  re-applies it.
- **Both laws on every Reading.** Tests check admission and realization on
  every Reading they produce.
- **Census before and after.** An engine change runs `sylex_english`'s
  census before and after it
  ([`crates/english/README.md`](../english/README.md), Working rules).
