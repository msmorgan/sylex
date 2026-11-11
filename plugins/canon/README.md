# `plugins/canon/`

Cards written by hand in `.syl`. This directory is a prototype, not the card database.

## What it is for

- **Exercising `.syl`.** A construction in `plugins/builtin/` that a card can write is exercised
  by a real card here, directly or through an expansion, so it is tested against text that a card
  prints. Primitives that appear only inside expansions, rules rows and the inventories of
  subtypes and counters are not all called from here.
- **Arriving at the term language.** The terms here are the `.syl` that the semantics layer is
  expected to read and the spelling grammar is expected to emit. Writing them by hand, before
  either exists, is how that language gets designed and how its defects are found.

A card is here because it exercises something. The number of cards is evidence of what has been
exercised and is not a goal.

## What it is not

- **Not permanent.** Hand-authoring is a stage. A card's resolved term is to be compiled from its
  Oracle text by recovery ([architecture](../../docs/decisions/architecture.md), "Authority, canon
  and artifact"). When recovery exists, these files are what its output is compared against, and
  then it replaces them.
- **Not an authority.** Oracle text is. When a term and its card's text disagree, the term is
  wrong.

## What follows for work here

- Write the term recovery should produce from the card's text and its printed characteristics.
  A choice an author makes here is a choice recovery will have to make without an author.
- A term that needs knowledge the text does not carry is a finding about the design. Report it;
  do not settle it silently in the card.
- A wording that no term can state, or that two terms state equally well, is a defect in
  `plugins/builtin/` or in the design documents, and is fixed there.
