---
needs: [syl-single-face-card-call]
---

# Expressions copied for lack of a local binding

## Why

`.syl` has no local bindings (`docs/decisions/syl.md` §2), so a body that needs one value twice
writes the expression twice: melee, rampage and sunburst copy theirs, and so do cards that print
one value for both power and toughness. `theActor` carries `identity = "actor"`, a field on the
`template` kind (`crates/xtask/src/dsl_check/schema.rs`) that nothing reads, and the Actor has two
words at NounPhrase: the primitive `actor` and the Template `theActor`.

## Design

- `.syl` gains no local binding; a Template parameter is the binding. A value the text prints once
  is written once, through a Template whose wording prints it once; a value the text prints twice
  is written twice.
- Melee and rampage call `getsBoth`, writing their delta once. Cards whose text prints one value
  for both power and toughness do the same; a static-ability form gets a `getsBoth` at
  `StaticSpec`.
- The `Replacement` `doIf` takes an `otherwise` replacement, and sunburst [CR#702.44a] is one
  static ability with an `otherwise`. `colorsSpentToCast(thisSpell)` stays written twice, because
  the rule prints "for each color of mana spent to cast it" twice.
- Shadow, soulbond and other keywords whose rule prints a phrase twice stay written twice.
- The `template` kind has no `identity` field. The Actor has one word at NounPhrase, the primitive
  `actor`; the Template `theActor` is deleted and its uses respelled `actor`. The `Subject`
  constructor `actor(np)` is unchanged.

## Done when

- Melee, rampage and sunburst write each printed value once.
- The `template` kind has no `identity` field, `theActor` is gone and `actor` carries its spelling.
- `docs/decisions/syl.md` §2 states the rule.
- Declaration order stays the printed order (`syl.md` §5).
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Not in scope

- Whether a Template argument used twice in a body is read once or twice (`syl-opaque-templates`).
- `theRingTemptsYou`'s copies (`syl-ring-tempts-you-raw-scope`).
- *Engineered Explosives* naming its sunburst (`syl-engineered-explosives-sunburst`).

## Pointers

`plugins/builtin/definitions/keyword_ability/{melee,rampage,sunburst}.syl`,
`plugins/builtin/templates/{action,ability}/getsBoth.syl`,
`plugins/builtin/primitives/static/doIf.syl`, `plugins/builtin/primitives/pronoun/actor.syl`,
`crates/xtask/src/dsl_check/schema.rs` (`template`), `docs/decisions/syl.md` §2,
`docs/decisions/semantics.md` §2.1.
