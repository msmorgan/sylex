---
needs: []
---

# SpecifyingClause uses a single-word flag on type and bypasses VerbPhrase

## Why

`docs/design/english.md` §3 forbids word flags: no feature or flag names a single open-class
word to license an attachment or construction, except a Catalog's license feature. `NounEntry::variable` in `nouns.ron` is set on
`"type"` alone, projecting `feature Variable = Cited` onto `Nominal` and `NounPhrase` across the
chart solely to license `SpecifyingClause` and `SpecifyingModalClause` for *Artificial Evolution*
(*the new creature type can't be Wall*). Both Productions also bypass `VerbPhrase` as flat
clauses, excluding other auxiliaries, adjuncts and coordinated values, and a Subject headed by
another taxonomic noun (*subtype*, *supertype*) is refused.

## Design

- Delete `NounEntry::variable` (`nouns.ron`, `schema.rs`) and `feature Variable = Cited`
  (`english.fug`), with no noun-class flag replacing them (variable/value sort matching belongs to
  `.syl`, as `"X can't be 0"` already parses through copular *be*).
- Build specifying copular clauses through a `VerbPhrase` Production headed by `Class(BeVerb)` with
  `head.Number != Plural` taking a singular cited-word complement (`Mention` or dedicated specifying
  complement — not `FrameSpec::Copular` shared with *become*/*remain*, and not general `Predicative`
  where `coordination Predicative` would mix `NominalPredicative` *"a Demon"* with bare subtypes on
  *Ancient Cellarspawn*), retiring flat `SpecifyingClause` and `SpecifyingModalClause`:
  `CliticSubjectPredicate`'s `subject.Case = Nominative` refuses *Urza's Tower*'s `"an Urza's
  Power-Plant"`, and `head.Number != Plural` refuses *Conversion*'s `"All Mountains are Plains"`.

## Done when

- `NounEntry::variable` and `feature Variable = Cited` are deleted, and no feature or flag marks
  `"type"` or a taxonomic noun class.
- Specifying copular clauses build through `VerbPhrase` instead of separate flat modal and non-modal
  `Clause` Productions.
- *Artificial Evolution* (*the new creature type can't be Wall*) reads, and *Conversion* (*All
  Mountains are Plains*), *Urza's Tower* (*Urza's Power-Plant*) and *Ancient Cellarspawn* (*that's a
  Demon, Horror, or Nightmare*) gain no spurious specifying Reading.
- The Census shows no verdict loss on the supported corpus, and every Corpus Unit whose Readings
  change is checked.
- `cargo xtask english strata --check`, `cargo test -p sylex_vocabulary -p sylex_english` and
  `cargo clippy -p sylex_vocabulary -p sylex_english --tests -- -D warnings` pass.

## Pointers

- `docs/design/english.md` §3, §6, §8.
- `crates/english/grammar/english.fug`: `feature Variable`, `SpecifyingClause`,
  `SpecifyingModalClause`, `Predicative`.
- `crates/vocabulary/lexicon/english/nouns.ron`: `type`.
- `crates/vocabulary/src/schema.rs`: `NounEntry::variable`.
