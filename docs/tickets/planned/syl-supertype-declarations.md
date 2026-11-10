---
needs: [syl-single-face-card-call]
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

## Design

- A `supertype` declaration kind, a catalog at `Supertype` like `card_type`, whose declarations
  carry their rules in `confers`.
- `basic`, `legendary`, `snow` and `world` are declared with it, and their primitives are gone.
- `legendary` writes the legend rule: if two or more legendary permanents with the same name are
  controlled by the same player, that player chooses one of them, and the rest are put into their
  owners' graveyards [CR#704.5j]. It also writes that a player can't cast a legendary instant or
  sorcery spell unless they control a legendary creature or a legendary planeswalker
  [CR#205.4e].
- `world` writes the world rule: if two or more permanents have the supertype world, all except
  the one that has had it for the shortest time are put into their owners' graveyards, and all of
  them on a tie [CR#704.5k].

## Open questions

- How the legend rule writes "with the same name" and "that player chooses one of them", and the
  world rule "had the world supertype for the shortest amount of time": `shares(cardName, …)`
  (`primitives/predicate/shares.syl`, `primitives/qualitySort/cardName.syl`) compares one object
  with another, and the state-based rows today (`card_type` `stateBased` conferrals,
  `rules/sba/deathtouchDestruction.syl`) each test one object, so none groups the permanents that share a name for `choose`
  (`primitives/action/choose.syl`) to pick from, and nothing records how long an object has had
  a supertype.
- What `basic` and `snow` carry. Their rules only define the quality: a basic land is a land with
  the supertype [CR#205.4c], a snow permanent a permanent with it [CR#205.4g]. Other rules read
  them from elsewhere: deck construction's basic-land exception [CR#100.2a] and the {S} cost paid
  with mana from a snow source [CR#107.4h]. Whether either belongs on the declaration.
- Whether the `CardType` and `Subtype` to `NounWord` coercions (`syl-implicit-coercions`) cover
  supertypes. A supertype is printed before the card types [CR#205.4a] and is never the head noun
  ("legendary creature", "basic land", "snow permanent"), so the expected answer is that no
  supertype coercion is needed.

## Done when

- `supertype` is a kind in `crates/xtask/src/dsl_check/schema.rs`.
- The four supertype primitives are replaced by `supertype` declarations, and every caller
  (`hasSupertype(…)`, `supertypes = […]`) still resolves.
- The legend rule and the world rule are written, and so is the legendary casting restriction.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass;
  `cargo xtask cite check --list-noncompliant` is empty and `cargo xtask cite check` reports
  0 stale, after `cargo xtask cite bless` for newly cited rules.

## Pointers

`plugins/builtin/primitives/supertype/`, `plugins/builtin/definitions/card_type/` (the `confers`
form), `plugins/builtin/rules/sba/`, `crates/xtask/src/dsl_check/schema.rs` (`card_type`,
`subtype`).
