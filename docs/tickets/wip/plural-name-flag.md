---
needs: [english-other-card-names]
---
# A card's `.syl` carries a `plural_name` flag, set from its own text's agreement

## Why

A card name has the Number of what it names: most are singular, and some take plural agreement in
their own Oracle text (*When The Notary Hobbits enter*, *Whenever Bebop & Rocksteady attack or
block*). Today `crates/vocabulary/catalogs/plural_names.ron` lists those names by hand. Rendering a
card from `.syl` still has to choose the agreement, so the number belongs on the card.

## Design

- The card's `.syl` carries a `plural_name` flag, and rendering reads it to choose agreement.
- The parse-to-`.syl` pipeline sets the flag from the agreement the card's own text shows; no list
  of plural names is kept. A probe that made every card name number-neutral grew derivations by
  0.04%, and every listed plural name came out `Number=Plural` from its verb's agreement, so the
  number can be read off the parse. The same probe let another card's name read as a plural
  subject in six units (*Exile target …* on Excise), which `english-other-card-names` prevents.

## Done when

- The flag exists on card declarations, the pipeline sets it, and rendering uses it.
- Card names are number-neutral in the parser, and every card in today's `plural_names.ron` gets
  the flag. `plural_names.ron` is deleted.

## Pointers

- `crates/vocabulary/src/lib.rs` (`name_numbers`), `crates/vocabulary/catalogs/plural_names.ron`
- `crates/english/grammar/english.fug` (`category Name`)
- `docs/contexts/oracle-english/CONTEXT.md` (Name Number)
