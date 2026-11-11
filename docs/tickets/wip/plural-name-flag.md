---
needs: [english-other-card-names]
---
# A card name is number-neutral in the parser

## Why

A card name has the Number of what it names: most are singular, and some take plural agreement in
their own Oracle text (*When The Notary Hobbits enter*, *Whenever Bebop & Rocksteady attack or
block*). A list of plural names declares that Number before any card is parsed, though only the
card's text shows it.

## Design

- No name declares a Number. Its Lexeme is one bundle unmarked for Number, and the Subject–verb
  agreement its text shows fixes the Number in the Reading, at the agreeing Clause; where the
  text shows none, the Number stays unvalued in one Reading (`docs/decisions/lexicon.md` §6).
- Another card's name stays unreadable as a Subject (*Exile target attacking creature* on
  Excise), through the opt-in of `english-other-card-names`.

## Done when

- `plural_names.ron` and every path that declares a name's Number are gone, and no Catalog name
  carries a Number.
- Each name the deleted list held parses with a Plural agreeing Clause in its own text.
- `crates/english/tests/name_number.rs` shows agreement fixing a name's Number (The Notary Hobbits
  Plural, Mizzium Mortars Singular) and an unmarked name left unvalued in one Reading (Falcon and
  Redwing); `other_card_names.rs` shows another card's name is no Subject in the real lexicon,
  with the control.
- `cargo xtask english strata --check` passes with no unit lost, and the derivation delta is
  reported.

## Not in scope

- Spelling a card name's agreement from its `.syl`: `syl-card-name-agreement`.

## Pointers

- `crates/vocabulary/src/lib.rs`, `crates/english/grammar/english.fug` (`category Name`)
- `docs/decisions/lexicon.md` §6, `docs/contexts/oracle-english/CONTEXT.md` (Name Number)
