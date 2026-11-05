---
needs: []
---

# Provenance has three kinds: Core, Catalog, Symbol

**Why.** `docs/decisions/lexicon.md` §3 gives a Lexeme's Provenance three kinds: Core (ordinary
English declared in `sylex_vocabulary`), Catalog (game vocabulary) and Symbol (notation symbols).
The Catalog constraint (`docs/decisions/english.md` §2) needs only the line between ordinary
English and game vocabulary. `SourceKind` in `crates/lexicon/src/model.rs` still has five kinds
(`Core`, `Plugin`, `Catalog`, `Keyword`, `Symbol`), so the code draws distinctions the design does
not have.

**Done when.**
- `SourceKind` has exactly `Core`, `Catalog` and `Symbol`; every `Plugin` and `Keyword` source is
  `Catalog`.
- Every construction site and test is updated (`crates/english/src/cleave.rs`, the `sylex_english`
  and `sylex_lexicon` tests, `sylex_vocabulary`'s loaders), and the Catalog-constraint check
  (`crates/english/tests/catalog_leak.rs`) still passes.
- `cargo xtask english strata --check` passes, and tests and clippy pass for `sylex_lexicon`,
  `sylex_vocabulary`, `sylex_english` and `xtask`.

**Pointers.** `crates/lexicon/src/model.rs` (`SourceKind`), `docs/decisions/lexicon.md` §3, the
Provenance entry in `docs/contexts/oracle-english/CONTEXT.md`.
