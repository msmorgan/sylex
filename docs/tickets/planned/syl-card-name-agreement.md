---
needs: [syl-self-name-references, syl-spelling-conventions, plural-name-flag]
---
# Spelling renders a card name's agreement from the card's `.syl`

## Why

A card name is number-neutral in the parser: the agreement its own Oracle text shows fixes its
Number (*When The Notary Hobbits enter*, *Mizzium Mortars deals*). Spelling renders a `.syl` term
to English and leaves inflection and agreement to the English layer (`spelling.md` §1), which
inflects a verb by its Subject's Number. When the Subject is the card's own name, the self-name
form of `syl-self-name-references`, nothing in the term says that Number, so spelling cannot
choose *enter* or *enters*.

## Design

- A face carries a typed Number field beside `name` (a Number value, not a `Bool`;
  `syl-boolean-flags`), written only on a face whose name is plural. Spelling reads it to inflect a
  verb whose Subject is the face's own name.
- In a canon `.syl` card the field is authored by hand. A canon card is a prototype of what
  spelling recovery will produce from Oracle text, so the field stands in for the Number recovery
  will derive from the face's own agreement, for as long as canon has to survive.
- The field is checked against the agreement the face's own Oracle text shows, so it is never
  written apart from that text.

## Done when

- Spelling a face's own name as a Subject inflects its verb by the name's Number: plural on every
  face whose own text shows plural agreement for its name (*The Notary Hobbits*, *Bebop &
  Rocksteady*, *The Neutrinos*), singular elsewhere.
- Every canon face's Number field matches the agreement its own Oracle text shows, and the check
  refuses a field the text does not show.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Pointers

- `docs/decisions/spelling.md` §1, §4, `docs/decisions/syl.md` §2, `docs/decisions/lexicon.md` §6
- `docs/tickets/planned/syl-self-name-references.md`
