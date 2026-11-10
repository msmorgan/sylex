---
needs: [syl-single-face-card-call, syl-card-type-permanent-type]
specs: [confers-and-rules]
---

# Supertypes as declarations that carry their rules

## Why

Card types and subtypes are catalog declarations (`card_type`, `subtype`) that carry `confers` and
their rules, but the supertypes are bare primitives
(`plugins/builtin/primitives/supertype/{basic,legendary,snow,world}.syl`) that carry nothing. The
rules the CR attaches to supertypes are written nowhere: the legend rule [CR#205.4d,704.5j], the
casting restriction on legendary instants and sorceries [CR#205.4e], and the world rule
[CR#205.4f,704.5k]. `plugins/builtin/rules/sba/` holds only `deathtouchDestruction`.

The supertypes are basic, legendary, ongoing, snow and world [CR#205.4a]. Ongoing's only rule concerns
scheme cards [CR#205.4h] and no Vintage-legal card has it, so it is out of scope; the other four
each appear on Vintage-legal cards.

This ticket lands after `syl-card-type-permanent-type`: both edit the kinds table in
`crates/xtask/src/dsl_check/schema.rs`, and the new `supertype` kind sits beside `card_type`.

## Design

- A `supertype` declaration kind, a catalog at `Supertype` like `card_type`, whose declarations
  carry `lexical`, `spelling`, `name` and `confers`.
- `basic`, `legendary`, `snow` and `world` are declared with it, and their primitives are gone.
- `basic` and `snow` carry only `lexical`, `spelling`, `name` and a citation: a basic land is a
  land with the supertype [CR#205.4c], a snow permanent a permanent with it [CR#205.4g]. The
  deck-construction exception for basic lands [CR#100.2a] belongs to the deck limit, and paying
  {S} with mana from a snow source [CR#107.4h] to the {S} symbol.
- `legendary`'s `confers` writes the casting restriction: a player can't cast a legendary instant
  or sorcery spell unless they control a legendary creature or a legendary planeswalker
  [CR#205.4e].
- The legend rule [CR#205.4d,704.5j] is not written on `legendary`. It is a named top-level `sba`
  row, `legendRule`, scoped to players (spec §4): "legend rule" is printed vocabulary, so it is its
  own declaration. Top-level rows (`sba`, `conferral`) take `lexical` and `spelling` and become
  catalog declarations.
- `legendRule` groups the player's legendary permanents by shared name, the player chooses one per
  group, and the rest are put into their owners' graveyards, in the group-choose-rest shape
  *Celestial Judgment* writes (`doForEachKind` with `choose`, then `notChosen`), with a new
  `KindAxis`, `name`.
- `world`'s `confers` writes the world rule [CR#205.4f,704.5k] as a per-holder `stateBased`
  entry: a world permanent is put into its owner's graveyard if another world permanent has had
  the world supertype for no longer than it has (spec §6). It reads a new measure, `heldFor`: how
  long an object has continuously satisfied a predicate. It is not a timestamp, because a
  permanent can gain the world supertype after it enters.
- `heldFor` is written at whichever sort expresses [CR#704.5k] exactly, including its tie case
  (all go when two tie for the shortest time): as a `ProjAxis` read by the existing
  `superlative`/`tiedFor` pattern (`bolster.syl`, `dethrone.syl`) if that says it, otherwise as an
  `Amount` compared with `compareOver`/`compareAmt`. The landing states which and why.
- No `Supertype` to `NounWord` coercion: a supertype is printed before the card types
  [CR#205.4a], and Oracle text uses one only as a premodifier ("legendary creature", "basic land")
  or a predicate adjective ("is no longer snow"), never as a head noun outside reminder text.

## Done when

- `supertype` is a kind in `crates/xtask/src/dsl_check/schema.rs`.
- The four supertype primitives are replaced by `supertype` declarations, and every caller
  (`hasSupertype(…)`, `supertypes = […]`) still resolves.
- `sba` and `conferral` rows take `lexical` and `spelling` and are catalog declarations.
- `rules/sba/legendRule.syl` writes the legend rule, `world` writes the world rule, and
  `legendary` writes the casting restriction; `name` at `KindAxis` and `heldFor` exist.
- The glossary entries **Conferral** and **Registry Definition**
  (`docs/contexts/game-model/CONTEXT.md`) list Supertype.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass;
  `cargo xtask cite check --list-noncompliant` is empty and `cargo xtask cite check` reports
  0 stale, after `cargo xtask cite bless` for newly cited rules.

## Not in scope

- The `rules` field on declarations: `syl-declaration-rules-field`.
- `doesntApply`, the word for "the "legend rule" doesn't apply" (spec §5): no card in the corpus
  prints it yet.
- Removing the engine's own copy of the legend rule (`crates/engine/src/sba.rs`).

## Pointers

`docs/specs/confers-and-rules.md`, `plugins/builtin/primitives/supertype/`,
`plugins/builtin/definitions/card_type/` (the `confers` form), `plugins/builtin/rules/sba/`,
`plugins/canon/cards/Celestial Judgment.syl`, `plugins/builtin/primitives/action/doForEachKind.syl`,
`plugins/builtin/primitives/condition/heldSince.syl`, `crates/xtask/src/dsl_check/schema.rs`
(`card_type`, `subtype`, `rules_row`).
