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

A type word in a description is not the type test alone. A description with a card type or
subtype that refers to no zone and has none of the words "card", "spell" or "source" means a
permanent of that type on the battlefield [CR#109.2]; "card" with a zone, "spell" and "source"
give it other domains [CR#109.2a,109.2b,109.2c]. "Target creature" is a creature permanent, and
"a creature card in your graveyard" is not one. The coercion's body, `hasType(creature)`, is the
type test alone, but so is `isCreature`'s today, and nothing in a term supplies the battlefield:

- Every determiner Template (`target`, `a`, `each`, `bare`, `allOf`, `the`, `counted`, `countOf`,
  `exists`) is `described(<det>, $p)` with nothing added.
- The corpus writes the domain only where the text prints it. Of the 1,070 type-predicate sites
  under `plugins/` (the nine `isX`, `hasType`, `hasSubtype`, `hasSupertype`), 41 carry
  `isPermanent`, `in(battlefield)` or `tokenOnTheBattlefield`, each where the text or the rule
  prints "permanent" or "on the battlefield" (the legend and world rules, storied, populate,
  bargain, forage, *Skred*'s "snow permanents", *Topple*'s "creatures on the battlefield"). 37
  carry `isCard` or `cardIn`, 61 `isSpell`, 28 a zone without "card", 6 `isToken` alone, and
  897 no domain word at all (190 under `target`, 172 `allOf`, 158 `a`, 80 `enchant`).
- `semantics.md` §2.4 reads [CR#109.2]'s carrier words for anaphors and lets a present-tense
  guard over a description with no zone or "card" read the permanent, and core keeps
  `Class(Permanent)` apart from the type atom (`crates/core/src/filter.rs`). No ADR says which
  stage turns "target creature" into a description of battlefield objects.

That gap predates these coercions, which change no expanded tree and inherit it unchanged.

## Design

- Three coercions under `plugins/builtin/coercions/`: `CardType` to `Predicate` through `hasType`,
  `Subtype` to `Predicate` through `hasSubtype`, `Supertype` to `Predicate` through
  `hasSupertype`. `Supertype` has no coercion into `NounWord`, since a supertype is no head noun;
  that reason does not apply at `Predicate`, where a filter marks no head noun and a supertype is a
  conjunct like a card type (*Arcum's Weathervane*'s "nonsnow basic land" is
  `and([land, basic, not(snow)])`).
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

- **Where the [CR#109.2] permanent reading lives.** The rule is a property of a whole description,
  keyed to which words it includes, not to a word's position: it holds for a subtype as for a
  card type, and a zone or "card", "spell" or "source" anywhere in the description cancels it.
  "Creature token" is a permanent, though "token" heads it, and "creature card" is not one.
  - (a) The body stays `hasType(x)`, and the default is applied to a description: a description
    of an object whose conjuncts, after expansion, include a type or subtype test and none of
    `isCard`, `cardIn`, `isSpell`, `isSource` or `in(<zone>)` describes battlefield objects. It
    applies where an object description is formed (`described`, and positions such as
    `addsTarget` that take a description directly), not to a complement (`is(it, creature)` tests
    an object already found). Expansion first lets a Definition's own words count: affinity's
    "you control" [CR#702.41a], splice's "spell" [CR#702.47a], and craft's "permanents you
    control and/or cards in your graveyard" [CR#702.167a]. No site is respelled, the round trip
    is unchanged, and no card's term changes meaning. `and([creature, isCard])` stays coherent,
    and `non(creature)` in *Dovin's Veto*'s "noncreature spell" stays a type test. The default
    is right only where printed "card" and "spell" are written. `syl-card-head-noun-sweep`
    restores most of the dropped ones. *Collected Company*, *Commune with the Gods* and
    *Tezzeret's Gatebreaker* (`fromAmong`) and *Skill Borrower* and *Conspicuous Snoop* (`is`)
    also drop a printed "card" and are not on its list.
  - (b) The body is `and([hasType(x), isPermanent])`. That is wrong wherever the type word
    modifies another carrier word. *Animate Dead*'s `and([creature, isCard, in(graveyard)])`
    describes nothing, and so do the other 36 card sites, 61 spell sites and 28 zone sites.
    `non(creature)` admits every spell. `or([artifact, creature, enchantment])` in *Custodi
    Squire*'s "artifact, creature, or enchantment card" admits no card. The modifier use needs
    another spelling, and since `(name, sort)` is unique, it goes back to `hasType(creature)` at
    the 126 card, spell and zone sites and at the negated sites outside a "permanent" phrase
    (*Meddling Mage*'s "nonland card name"). The word "creature" then has two terms, picked by a distinction the CR does
    not draw.
  - (c) Two words, one for a head noun and one for a modifier. A word can't be declared twice at
    `Predicate`, so this means giving each description a head `NounWord` (which already has
    `card`, `spell`, `permanent`, `token`, `source`, `type`, `subtype` and `ofType`) beside a
    modifier `Predicate`. Every determiner Template and all 1,070 sites change, and the head
    still doesn't decide the domain ("creature token"). In a flat `and([…])`, printed order
    (`syl-list-element-printed-order`) puts the head last among the conjuncts printed before
    it. Which conjuncts those are comes from each one's Construction, so a checker could
    recover the head, but [CR#109.2] doesn't need it.
  - (d) No better placement was found. A word-level body cannot see the description's other
    words, and the rule depends on them.

  Recommendation: (a). The discourse reading records each description's domain from its
  conjuncts under [CR#109.2,109.2a,109.2b,109.2c] (§2.4 already needs a referent's zone for
  carrier words), lowering emits `Class(Permanent)`, and `semantics.md` says so. The coercions
  don't wait on that work, since they change no expanded tree. Under (b) or (c) this ticket
  would be redesigned.
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

- `docs/decisions/syl.md` §2 and §3; `docs/decisions/semantics.md` §2.4
- `syl-card-head-noun-sweep`
- `crates/xtask/src/dsl_check.rs` (`value_at`, `coerce`, `parameter`),
  `crates/xtask/src/dsl_check/coercion.rs` (`coercions`, `read`, `ambiguities`)
- `plugins/builtin/coercions/`, `plugins/builtin/primitives/predicate/{hasType,hasSubtype,hasSupertype}.syl`,
  `plugins/builtin/templates/predicate/`
