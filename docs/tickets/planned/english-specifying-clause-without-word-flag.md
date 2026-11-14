---
needs: []
---

# SpecifyingClause uses a single-word flag on type and bypasses VerbPhrase

## Why

`docs/design/english.md` §3 forbids word flags: no feature or flag names a single open-class
word to license an attachment or construction. `NounEntry::variable` in `nouns.ron` is set on
`"type"` alone, projecting `feature Variable = Cited` onto `Nominal` and `NounPhrase` across the
chart solely to license `SpecifyingClause` and `SpecifyingModalClause` for Artificial Evolution
(*the new creature type can't be Wall*). Both Productions also bypass `VerbPhrase` as flat
clauses, excluding other auxiliaries, adjuncts and coordinated values, and a Subject headed by
another taxonomic noun (*subtype*, *supertype*) is refused.

## Open questions

- What general mechanism should license specifying copular clauses with cited subtype/type values
  on `VerbPhrase` (for example, a taxonomic noun class or distribution shared by *type*, *subtype*
  and *supertype*, or a copular cited-value complement gated against bare plural-syncretic
  subtypes as in Conversion's *All Mountains are Plains*)?

## Done when

- `NounEntry::variable` and `feature Variable = Cited` are deleted, and no feature or flag marks
  `"type"` alone.
- Specifying copular clauses build through `VerbPhrase` (or the structure the owner rules on)
  instead of separate flat modal and non-modal `Clause` Productions.
- Artificial Evolution (*the new creature type can't be Wall*) reads, and Conversion (*All
  Mountains are Plains*) and Urza's Tower (*Urza's Power-Plant*) gain no spurious specifying
  Reading.
- The Census shows no verdict loss on the supported corpus, and every Corpus Unit whose Readings
  change is checked.
- `cargo test -p sylex_vocabulary -p sylex_english` and
  `cargo clippy -p sylex_vocabulary -p sylex_english --tests -- -D warnings` pass.

## Pointers

- `docs/design/english.md` §3, §6, §8.
- `crates/english/grammar/english.fug`: `feature Variable`, `SpecifyingClause`,
  `SpecifyingModalClause`.
- `crates/vocabulary/lexicon/english/nouns.ron`: `type`.
- `crates/vocabulary/src/schema.rs`: `NounEntry::variable`.
