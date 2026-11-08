---
needs: [english-other-card-names]
---

# Eliminate free-variation spelling variants (`LexicalValue::variant`)

## Why

Oracle text has no free variation: conditioned allomorphs (`a`/`an`, `non`/`non-`, `'s`/`'`) are
deterministic, not interchangeable spellings of a single form. Storing multiple `surfaces` per `FormDeclaration` and carrying `LexicalValue::variant`
on grammar leaves lets the parser accept mismatched allomorphs (`non-land`, `nonVampire`, `a
artifact`, `players's`) and echo the input buffer back on realization instead of deriving the
surface from grammatical and lexical properties.

## Done when

- `FormDeclaration` carries a single optional surface override (`Option<String>`),
  `LexicalValue::variant` and the `Spelling Variant` glossary entry are removed, and every `(lexeme,
  form, features, capitalization)` tuple in `sylex_lexicon` realizes to a unique string.
- The curly-apostrophe duplicates in `closed_class.ron` (`'ve`, `'re`, `genitive`) are deleted. No
  in-scope Oracle text contains a curly quote or apostrophe, so nothing normalizes them.
- `non` vs. `non-` is selected by a lexical feature on the host (shared by subtypes and `outlaw`)
  rather than a capitalization rule or `spellings: ["non", "non-"]`.
- Indefinite article allomorphy (`a` vs. `an`), genitive clitic allomorphy (`'s` vs. `'`), and
  shortened card names (`[CR#201.5c]`) are modeled without multi-surface form variants, with
  negative judgments pinning rejected allomorph pairings.
- `Spawn` has the zero plural, as in *Hand of Emrakul*'s `four Eldrazi Spawn` (the set that
  introduced the type, kept through Oracle rewording), and *Azlask, the Swelling Scourge*'s `Scions
  and Spawns` is corrected to `Scions and Spawn` in `crates/english/corrections.txt`.
- `cargo xtask english strata --check` passes.

## Pointers

- `crates/lexicon/src/types.rs` (`LexicalValue`)
- `crates/lexicon/src/model.rs` (`FormDeclaration::surfaces`)
- `crates/lexicon/src/analysis.rs` (`Lexicon::new`, `Lexicon::realize`)
- `crates/vocabulary/lexicon/english/closed_class.ron` (`a`, `'ve`, `'re`, `non`, `genitive`)
- `crates/vocabulary/catalogs/subtypes.ron` (`Spawn`)
- `crates/english/corrections.txt` (*Azlask, the Swelling Scourge*)
- `crates/vocabulary/src/names.rs` (shortened card names)
- `crates/english/grammar/english.fug`
- `docs/decisions/english.md`, `docs/decisions/lexicon.md`,
  `docs/contexts/oracle-english/CONTEXT.md`
