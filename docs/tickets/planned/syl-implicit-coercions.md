---
needs: [syl-primitive-constructors, syl-single-face-card-call]
---

# Declared coercions in `.syl`

## Why

After `syl-primitive-constructors`, `.syl` has no implicit conversions besides `Option`
(`docs/decisions/syl.md` §2): a value of one sort reaches a position of another only through a
wrapper written at the call site. A card type at a head noun is reached through eight one-line
Templates (`templates/noun/{artifact,battle,creature,enchantment,instant,kindred,land,sorcery}.syl`,
each `type(<card type>)`), a subtype only through a written `subtype(…)`, and an integer at
`Amount` only through a written `lit(…)`. `syl.md` §2 leaves room for conversions to return
deliberately once the language is simple enough.

It belongs in the spine because every later ticket writes bare type and subtype head nouns and
bare integers; with the coercions declared first, each writes them once, as printed.

## Design

- A coercion is its own declaration kind, `coercion`, with no name:
  `coercion(type: CardType): NounWord { body = type($type) }`. Its body wraps an existing
  primitive, so it builds no new constructor, and the wrapped primitive stays callable.
- There are three: `CardType` to `NounWord` through `type`, `Subtype` to `NounWord` through
  `subtype`, and `Int` to `Amount` through `lit`.
- Mana symbols stay explicit (`pip`, `colorlessPip`, `generic`): a color word is not a mana
  symbol.
- A coercion applies only at a sort mismatch, and only one hop: a coerced value is never coerced
  again.
- `cargo xtask dsl check` refuses two coercions with the same from-sort and to-sort, and a word
  that would resolve at one expected sort both directly and through a coercion.
- `Option` composes with a coercion as it does with any value today: at an `Option(T)` position
  the value is written as a `T`, and a coercion into `T` applies.

## Open questions

- Whether the existing explicit `lit(…)`, `type(…)` and `subtype(…)` call sites are respelled
  later to rely on the coercion. They stay valid either way.

## Done when

- The `coercion` kind is in `cargo xtask dsl check`, and in `sylex_syl`'s parser if a nameless
  declaration needs it; the three coercions are declared under `plugins/builtin/`.
- The eight head-noun Templates
  `templates/noun/{artifact,battle,creature,enchantment,instant,kindred,land,sorcery}.syl` are
  deleted, and their callers resolve through the coercion unchanged.
- Negative tests refuse an ambiguous coercion (a word that resolves both directly and through a
  coercion), a chained coercion and a duplicate coercion.
- `docs/decisions/syl.md` §2 and §3 state the coercion rule in place of "no conversions besides
  `Option`" and "the one implicit wrap the language keeps", and §3's resolution order says where a
  coercion is tried.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Pointers

`docs/decisions/syl.md` §2 and §3, `crates/xtask/src/dsl_check.rs` (`Resolver`),
`crates/xtask/src/dsl_check/schema.rs` (`KINDS`), `crates/syl/src/parse.rs`,
`plugins/builtin/primitives/nounWord/{type,subtype}.syl`,
`plugins/builtin/primitives/quantity/lit.syl`, `plugins/builtin/templates/noun/`.
