---
needs: [syl-primitive-constructors, syl-single-face-card-call]
---

# Declared coercions in `.syl`

## Why

After `syl-primitive-constructors`, `.syl` has no implicit conversions besides `Option`
(`docs/decisions/syl.md` §2): a value of one sort reaches a position of another only through a
wrapper written at the call site. A card type at a head noun is reached through nine one-line
Templates (`templates/noun/{artifact,battle,creature,enchantment,instant,kindred,land,planeswalker,sorcery}.syl`,
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
- `cargo xtask dsl check` refuses two coercions with the same from-sort and to-sort.
- For an expected sort `T`, Reaches(`T`) is `T` together with every sort `S` that has a coercion
  `S` to `T`. A word declared at two sorts in Reaches(`T`) is refused: a word at both `CardType`
  and `Subtype` (both reach `NounWord`), and a word declared directly at `NounWord` and also at
  `CardType`, so a direct declaration never silently shadows a coercion (a leftover
  `templates/noun/creature.syl` beside `card_type creature`).
- Literals likewise: an integer literal that reaches `T` from both `Int` and `Nat` is refused,
  since one literal stands at both, and so is a record literal that reaches `T` from two record
  sorts.
- The ambiguity check runs at load, across the whole registry, not only where a call site writes
  the word, so a latent collision fails before any card uses it.
- `Option` composes with a coercion as it does with any value today: at an `Option(T)` position
  the value is written as a `T`, and a coercion into `T` applies.
- Every existing `lit(…)`, `type(…)` and `subtype(…)` call site whose wrap a coercion would
  supply is respelled bare, in one scripted commit gated by a check that each respelled file
  resolves to the same tree, so the corpus has one spelling of each term; the wrapped primitive
  stays callable.

## Open questions

- Whether `cargo xtask dsl fmt` collapses an explicit wrap a coercion would supply. The formatter
  reads one file's syntax and resolves no sort, and the wrap is not visible in the syntax:
  `subtype(goblin)` is the `NounWord` primitive at a head noun and the `TextWord` primitive in a
  text change, and only the expected sort tells them apart.

## Done when

- The `coercion` kind is in `cargo xtask dsl check`, and in `sylex_syl`'s parser if a nameless
  declaration needs it; the three coercions are declared under `plugins/builtin/`.
- The nine head-noun Templates
  `templates/noun/{artifact,battle,creature,enchantment,instant,kindred,land,planeswalker,sorcery}.syl` are
  deleted, and their callers resolve through the coercion unchanged.
- Negative tests refuse, each at load with no call site writing the word: a word declared at two
  from-sorts of one coercion target (`CardType` and `Subtype` into `NounWord`); a word declared
  both directly at a target and at a sort coerced into it; an integer literal reaching a target
  from both `Int` and `Nat`; a record literal reaching a target from two record sorts; a chained
  coercion; and a duplicate coercion.
- `docs/decisions/syl.md` §2 and §3 state the coercion rule in place of "no conversions besides
  `Option`" and "the one implicit wrap the language keeps", and §3's resolution order says where a
  coercion is tried.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Pointers

`docs/decisions/syl.md` §2 and §3, `crates/xtask/src/dsl_check.rs` (`Resolver`),
`crates/xtask/src/dsl_check/schema.rs` (`KINDS`), `crates/syl/src/parse.rs`,
`plugins/builtin/primitives/nounWord/{type,subtype}.syl`,
`plugins/builtin/primitives/quantity/lit.syl`, `plugins/builtin/templates/noun/`.
