---
needs: [cleanup-step6-spine]
---

# Card types, subtypes and supertypes stand bare as predicates

## Why

A card type, subtype or supertype in a filter is written through a wrapper: "legendary creature
you control" is `and([hasSupertype(legendary), isCreature, youControl])`, where the same words as
a head noun are already bare (`that(creature)`, through the `CardType` and `Subtype` coercions
into `NounWord`). The wrappers are the corpus's most common filter words: outside comments,
`plugins/` writes `isCreature` 643 times, `isLand` 120, `isArtifact` 79, `isEnchantment` 27,
`isPlaneswalker` 20, `isInstant` 5, `isSorcery` 4, `isBattle` 3 and `isKindred` 0, plus
`hasSubtype(…)` 121, `hasSupertype(…)` 29 and `hasType(…)` 19 (10 in the nine `isX` bodies):
about 1,080 sites, 199 in `plugins/builtin/` (121 files) and 880 in `plugins/canon/` (566 files).
Three more coercions let each be written as printed: `and([legendary, creature, youControl])`.

The mechanism already admits them:

- A coercion is picked by expected sort: `coerce` (`crates/xtask/src/dsl_check.rs`) keeps only
  coercions whose to-sort is the expected sort, and the duplicate check
  (`dsl_check/coercion.rs`, `coercions`) refuses only a repeated from-sort and to-sort pair, so
  `CardType` into both `NounWord` and `Predicate` is two coercions, never a choice between them.
  No chain forms: `Predicate` is no from-sort, and `CardType`, `Subtype` and `Supertype` are no
  to-sort.
- No slot admits both `NounWord` and `Predicate`. A sort is one nominal `SortKey` (no union
  sorts), a list item resolves at the list's element sort, and an `Option(T)` position resolves at
  `T`. A slot's sort fixes which tree a bare word builds: `target(p: Predicate)` and
  `that(w: NounWord)` both write `creature`, and build `HasType` and `Type` respectively.
- Reaches(`Predicate`) would be `Predicate`, `CardType` (10 words), `Subtype` (463) and
  `Supertype` (4: `basic`, `legendary`, `snow`, `world`). No word is declared at two of them, and
  none of the 130 `Predicate` words (`is*`, `has*`, `youControl` and the rest) is a type name:
  there is no `land`, `snow`, `basic`, `legendary`, `creature`, `spell` or `token` at
  `Predicate`, and nothing is declared at `Option(Predicate)`, so the shadow rule finds nothing.
  The `List(CardType)` Templates `creature` and `spell`, the `SubtypeSpace` primitive `creature`
  and the `ManaSymbol` `snow` lie outside Reaches(`Predicate`). `spell` stays unreachable at
  `Predicate`: it is no card type, so `isSpell` stays.
- Each body is an existing primitive at `Predicate` applied to the parameter, as `read` requires:
  `hasType(type: CardType)`, `hasSubtype(subtype: Subtype)`, `hasSupertype(supertype: Supertype)`
  (`plugins/builtin/primitives/predicate/`). The nine single-type Templates
  (`templates/predicate/is{Artifact,Battle,Creature,Enchantment,Instant,Kindred,Land,Planeswalker,Sorcery}.syl`)
  are each `hasType(<type>)` with the type's own spelling, so the coercion builds the same tree.
- Nothing printed is lost. The explicit form's only extra content is the primitive's name, and
  the word's sort already determines it, since no word stands at two sorts of
  Reaches(`Predicate`). The adjective/noun distinction the `NounWord` coercions guard is a head-noun
  matter; in a filter, a supertype and a card type are conjuncts alike, each spelled by its own
  word (`hasType`'s and `hasSupertype`'s spelling is the argument's). Printed order of the
  conjuncts is `syl-list-element-printed-order`'s.

## Design

- Three coercions under `plugins/builtin/coercions/`: `CardType` to `Predicate` through `hasType`,
  `Subtype` to `Predicate` through `hasSubtype`, `Supertype` to `Predicate` through
  `hasSupertype`.
- The nine single-type `isX` Templates are deleted and every call site respelled bare, so the
  corpus has one spelling of each term (`syl.md` §2). `hasType`, `hasSubtype` and `hasSupertype`
  stay callable, and their call sites at a coerced position are respelled bare too, a parameter
  read included (`hasSubtype($subtype)` in `amass.syl` becomes `$subtype`).
- Templates that are no card type stay, with their bodies respelled: `isInstantOrSorcery` is
  `or([instant, sorcery])`; `isPermanentType`, `isAnyTarget`, `isHistoric` and `isOutlaw` write
  their types bare; `isPermanent`, `isSpell`, `isCard`, `isPermanentCard`, `isToken` and
  `isNontoken` are unchanged.
- The respell is one scripted commit, gated by a check that each respelled file resolves to the
  same expanded tree.

## Open questions

- **Whether `Supertype` reaches `Predicate`.** `Supertype` has no coercion into `NounWord`, since
  a supertype is no head noun. That reason does not apply at `Predicate`: `hasSupertype`'s
  spelling is its argument's, a filter has no head, and the 29 `hasSupertype(…)` sites
  (`legendary` 12, `basic` 10, `snow` 4, `world` 3) are the sites where the wrapper most breaks
  printed reading (*Arcum's Weathervane*'s "nonsnow basic land" is
  `and([isLand, hasSupertype(basic), not(hasSupertype(snow))])`). Recommendation: include it, so the three type sorts read alike in a filter.
- **A double refusal.** A word declared at both `CardType` and `Subtype` now collides in
  Reaches(`NounWord`) and Reaches(`Predicate`), and `ambiguities` runs per target, so it is
  refused twice, once per target, at the same declaration. Recommendation: keep both, since each
  names a distinct position the word would be ambiguous at; the negative test asserts both.

## Done when

- The three coercions are declared, the nine `isX` single-type Templates are deleted, and no
  `hasType(…)`, `hasSubtype(…)` or `hasSupertype(…)` call remains at a position a coercion
  reaches (including inside the kept compound Templates).
- New tests in `crates/xtask/tests/suite/dsl_check.rs`: one from-sort with coercions into two
  to-sorts resolves a bare word at each by expected sort; a word at `Predicate` and at
  `Supertype` (or `CardType`, `Subtype`) is refused with no call site; a word at `Supertype` and
  at `CardType` is refused with no call site.
- `docs/decisions/syl.md` §2 lists six coercions, and §3's example of a word outside every
  Reaches set no longer names `snow` at `Supertype`, which now reaches `Predicate` (`snow` at
  `ManaSymbol` still collides with nothing); the comment in
  `a_word_at_two_sorts_no_coercion_joins_is_no_ambiguity` is corrected likewise.
- Planned tickets that spell a removed `isX` Template or a now-bare wrap
  (`syl-list-element-printed-order`, `syl-card-head-noun-sweep`, `syl-tocast-noun-phrase`,
  `syl-daybound-reconfigure-bodies`, `syl-deathtouch-creature-conferral`, and
  `docs/specs/confers-and-rules.md`) are respelled.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check`, `cargo test -p xtask` and
  `cargo clippy -p xtask --tests -- -D warnings` pass.

## Not in scope

- A `Supertype` coercion into `NounWord`.
- Reordering conjuncts into printed order: `syl-list-element-printed-order`.

## Pointers

- `docs/decisions/syl.md` §2 and §3
- `crates/xtask/src/dsl_check.rs` (`value_at`, `coerce`, `parameter`),
  `crates/xtask/src/dsl_check/coercion.rs` (`coercions`, `read`, `ambiguities`)
- `plugins/builtin/coercions/`, `plugins/builtin/primitives/predicate/{hasType,hasSubtype,hasSupertype}.syl`,
  `plugins/builtin/templates/predicate/`
