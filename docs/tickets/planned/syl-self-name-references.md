---
needs: [cleanup-step6-spine, syl-number-snapshot, syl-printed-it-self-references]
---

# A card that prints its own name writes a name form, not `this`

## Why

Canon cards write the bare `this` primitive where the card prints its own name: *Lightning
Bolt*'s "Lightning Bolt deals 3 damage to any target" is `instruct(this, dealDamage(…))`. Text
that names the object it's on means that object [CR#201.5], and some cards print a Shortened Name
instead, treated as the full name [CR#201.5c]: *Urza, Lord High Artificer*'s "When Urza enters".
One face can print both: *Immard, the Stormcleaver* says "Whenever Immard, the Stormcleaver
enters" and "Immard deals 4 damage to any target". Written as `this`, the printed name, the
Shortened Name and "this creature" are one term, so spelling can't render what the card prints
and nothing checks it.

`syl-printed-it-self-references` rewrites canon self-reference sites to `it` first, and
`syl-this-head-template` rewrites the sites that print "this <head>" after; all three rewrite
canon self-reference sites, so they land in that order.

## Design

- Two zero-argument NounPhrase forms name the card: one its full name, one its Shortened Name.
- Neither takes a string: the name is the enclosing face's, so an ability gained by an object
  with another name reads as that name [CR#201.5b].
- `cargo xtask dsl check` refuses the Shortened Name form on a face with no Shortened Name, and
  both forms anywhere in `plugins/builtin/`, where there is no face.

## Open questions

- The two forms' names; the glossary has **Shortened Name** but no term for the full self-name
  (`ownName` and `ownShortenedName`, or `selfName` and `shortenedSelfName`, are candidates).
- Whether they are two primitives, or two Templates over one primitive with a name-form argument
  (not a Bool).
- Where a face records its Shortened Name.
- Whether a granted ability that names its grantor by name [CR#201.5a] (*Grothama,
  All-Devouring*'s "fight Grothama, All-Devouring"; *Gutter Grime*'s token) gets the same
  name-versus-head split beside `theGrantor`.

## Done when

- Every canon mention where the card prints its full name or its Shortened Name uses the matching
  form, and the landing lists the faces that print a Shortened Name.
- Canon `this` sites that print "this spell" rather than a name (*Abolish*'s
  `altCost(this, …)`, and `costShift(this, …)` and `keywordCostPaid(kicker, this)` sites such as
  *Academy Journeymage*, *Embercleave* and *Conqueror's Pledge*) are left to `syl-this-head-template`.
- Negative tests refuse the Shortened Name form on a face without one, and either form in
  `plugins/builtin/`.
- The landing runs the `syl-number-snapshot` diff and explains every flagged line.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass;
  `cargo xtask cite check --list-noncompliant` is empty and `cargo xtask cite check` reports
  0 stale, after `cargo xtask cite bless` for newly cited rules.

## Not in scope

Sites that print "this <head>": `syl-this-head-template`.

## Pointers

`plugins/builtin/primitives/pronoun/{this_,theGrantor}.syl`, `plugins/canon/cards/`,
`docs/contexts/oracle-english/CONTEXT.md` (**Shortened Name**, **Proper Name**).
