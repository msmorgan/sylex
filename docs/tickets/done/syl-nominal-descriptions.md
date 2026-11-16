---
needs: [cleanup-step6-spine, syl-spelling-optional-segments, syl-spelling-conventions]
---

# Descriptions are nominals: a head with its pre- and postmodifiers

## Why

A description that includes a card type or subtype, refers to no zone and has none of the words
"card", "spell" or "source" means a permanent of that type on the battlefield [CR#109.2]; "card"
with a zone, "spell" and "source" give it other domains [CR#109.2a,109.2b,109.2c]. Nothing in a
term says so. A description is a flat `and([…])` of `Predicate`s, so "creature" in "target
creature" and "creature" in "creature card" are the same term, `isCreature` (`hasType(creature)`),
and the battlefield is in neither.

A survey of the checker's own resolution found 1,159 sites where a type, subtype or supertype word
reaches a `Predicate` (963 in canon cards, 196 in builtin bodies); 524 canon and 83 builtin sites
are a description's head with the permanent reading, and the rest a modifier, a coordination, a
complement, a keyword's argument, a supertype, or a type word the card does not print
(`docs/evidence/syl-type-word-templates/summary.txt`). A separate study counted 1,510 description
sites by its own parser, 1,341 of them respellable mechanically to a head and two modifier lists
(`docs/evidence/cleanup-step6-syl/head-noun-study.md`).

The head noun has a second, unrelated sort. `NounWord` holds the heads `card`, `permanent`,
`spell`, `token`, `player`, `opponent` and the coercions from `CardType` and `Subtype`
(`plugins/builtin/coercions/`), and `that`, `those`, `headed`, `enchanted`/`equipped`/`fortified`,
`theGrantor` and `reach/{word,unionHalf}` take it, as do the `NounWord → NounWord` constructors
`ofType` and `copied`. So "permanent" is both `permanent` at `NounWord` and `isPermanent` at
`Predicate`, and likewise `card`/`isCard` and `opponent`/`isOpponent`.

The rules rows scope by `Predicate` too: `ConferralRule.scope` and `SbaRule.scope` are the only
`"Predicate"` references in `crates/xtask/src/dsl_check/schema.rs` (lines 352 and 440).

## Design

- A description is a `Nominal` (glossary **Nominal**, matching `english.fug`): its head, with a
  `List(PreModifier)` before it and a `List(PostModifier)` after it, built by
  `nominal(pre = [], head, post = [])` (`spelling = "<$pre#<> >$head< $post#<>>"`). Each list's
  elements stand in printed order, and `pre` is declared before the required `head` because it is
  printed first (`syl.md` §2). `headed` is taken by an existing primitive ("$subject as a $head"),
  so the constructor is `nominal`.
- `cargo xtask dsl check` refuses `nominal(head)` with neither `pre` nor `post`, and refuses an
  uncoordinated `nominal(…)` directly inside `nominal.head`, so a nominal has one tree. A
  coordination at `Nominal` (`coord`, `and`, `or`, `andOr`) is a union of the domains its arms
  describe ("all artifacts and creatures"), whereas modifiers in `pre` and `post` intersect with the
  head; `nominal.head` may hold a coordination of nominals under shared postmodifiers ("Target white
  creature or black Zombie you control":
  `nominal(or([nominal(pre = [white], creature), nominal(pre = [black], zombie)]), post =
  [youControl])`). Where a nominal has two well-sorted attachments, canon writes the attachment each
  card's Oracle text means (`pragmatics.md` §4).
- Inside a `PreModifier` coordination, a stacked arm of two or more premodifiers is
  `stacked(modifiers: List(PreModifier))` (`spelling = "$modifiers#<>"`, `body = coord(and,
  $modifiers)`; *Acclaimed Contender*'s "Knight, Aura, Equipment, or legendary artifact card").
  `cargo xtask dsl check` refuses `stacked` with fewer than two elements or outside a `PreModifier`
  coordination arm.
- Eight coercions exist (`plugins/builtin/coercions/`): `CardType` and `Subtype` into `Nominal`
  (through `type` and `subtype`, each meaning a permanent of that type or subtype on the battlefield
  [CR#109.2,110.1]); `CardType`, `Subtype`, `Supertype` and `Color` into `PreModifier` (through
  `hasType`, `hasSubtype`, `hasSupertype` and `hasColor`, testing the characteristic alone
  [CR#105.1,205.4a]); `Status` into `Predicative` (through `hasStatus` [CR#110.5]); and `Int` into
  `Amount` (through `lit`). `Status` does not coerce into `PreModifier`, where "face-down" and
  "phased-out" spell with a hyphen through `PreModifier` Templates (`tapped`, `untapped`,
  `faceDown`, `phasedOut`) over an unspelled `hasStatus`. `DesignationLabel` has no coercion, since
  non-adjectival labels such as `monarch` and `citysBlessing` are not premodifiers; designations
  printed as premodifiers (`foretold`, `plotted`) get a `PreModifier` Template each.
- A copula's complement is a `Predicative`: what `is(subject, predicative)` takes ("If it's a land",
  "is tapped", "is in your graveyard"). Noun-phrase complements go through
  `noun(phrase: NounPhrase): Predicative = NominalPredicative` (`english.fug`), where [CR#109.2]'s
  battlefield restriction does not apply: a type word tests the characteristic alone in the
  subject's zone, and "card" or "permanent" adds that class (*Explore*, *Bioplasm*, *Skill
  Borrower*).
- `NounWord` is merged into `Nominal`: every former `NounWord` consumer (`attachHost`, `enchanted`,
  `equipped`, `fortified`, `headed`, `that`, `those`, `theGrantor`, `word`, `unionHalf`) takes a
  `Nominal`, and each head word means at `Nominal` what its description word meant (`opponent` is
  `opponentOf(you)` [CR#102.2,102.3]). `ofType` and `copied` are gone in favor of `nominal` with a
  premodifier (`nominal(pre = [creature], card)`, `nominal(pre = [copied], ability)`).
- Head semantics: `card` tests the card class alone, never a zone ([CR#108.2,108.2b,109.2a]);
  `permanent` is a card or token on the battlefield ([CR#110.1]), while a printed "on the
  battlefield" stays the postmodifier `in(battlefield)` (*Topple*, *Éomer of the Riddermark*,
  *Clone*, *Caller of the Hunt*);
  `token` tests token status alone ([CR#111.1,111.7]); `spell` and `source` give their domains
  ([CR#109.2b,109.2c,112.1,113.7]); an ability class (`abilityHead`) with a printed source takes the
  `PostModifier` `ofSource` (*Arrest*, *Cursed Totem*) while unqualified "abilities of …" uses the
  `Nominal` Template `abilityOf` over `ability` + `ofSource`, and "of the <axis> of your choice" on
  a permanent head uses `PostModifier` `ofYourChoice` (*Extinction*, *Root Greevil*). Descriptions
  in `play(bare(land))` ([CR#116.2a,305.1]),
  look-back triggers ([CR#603.10a]), entering permanents ([CR#614.12]), and departed objects
  ([CR#608.2h]) keep the permanent reading and are read at the event's or action's point in time
  (`semantics.md` §3.1).
- Agreement phrases printed among postmodifiers (`agree`, `withDifferentNames`, `withTheSameName`)
  stand at `PostModifier` in `nominal.post` in printed order (*Eerie Ultimatum*), lifted in lowering
  to constrain the enclosing plural group; `cargo xtask dsl check` refuses an Agreement
  `PostModifier` when its enclosing consumer forms no plural group.
- Operators whose own words span or attach across a head are Templates with Holes in printed order:
  `ordinalRelative` at `Nominal` (*Highspire Bell-Ringer*'s "second spell you cast each turn") and
  `hasLesserManaValueThan` at `PostModifier` (*Death's Oasis*'s "with lesser mana value than the
  creature that died").
- `VerbPhrase` does not coerce into `PostModifier` (`attachment` stands at both sorts); attributive
  pre-head participials ("the exiled card") use `was` at `PreModifier`, post-head relatives and
  participials use `did` and `was` at `PostModifier`, and copular lookbacks use `did` and `was` at
  `Predicative` until `syl-lookback-operator-and-holds`.
- `ConferralRule.scope` and `SbaRule.scope` take a `Nominal`, and all eight rules rows under
  `plugins/builtin/rules/` write nominal scopes.
- Non-description `Predicate` consumers — spend purposes (`toCast`, `toTurnFaceUp`, `toActivate`),
  `transform` / `turnOver`, `procedure.resulting`, keyword qualities (`protection`, `hexproofFrom`,
  `affinity`, `champion`, `behold`, `enchant`, `equipQuality`, `emergeFrom`, `devourQuality`,
  `bandsWithOther`, `offering`, `landwalk`, `typecycling`, `splice`, `partnerWith`,
  `primitiveKeyword`, `devourEntry.quality`), ability/holder filters (`allExcept`, `addedAbilitiesOf`,
  `abilityGrantFrom`, `allAbilityLoss`, `addedKeywordTerm`, `keywordGrant`, `heldFor`),
  `chosenQuality` / `withQuality`, `aCharacteristic`, `nameOfCard`, `players`, `spendOnlyToCastAnd`,
  and `oneEachOf` — stay at `Predicate` until their own tickets retype them.

## Done when

- The open questions are answered, and each answer is written where it governs: `spelling.md`,
  `syl.md`, `semantics.md` §2.4 and §3.1, and the declarations' doc comments.
- `spelling.md` describes a description as a nominal whose modifiers are substitution into its two
  lists: nothing in it describes a description as one `Predicate` list, and explicit premodifier
  and postmodifier slots are not among its rejected alternatives. `syl.md` §2 and §3 list the coercions that
  exist and give examples that do, and `syl.md` §5's "basic land" example is a nominal.
- `semantics.md` states [CR#109.2]: a type word at `Nominal` means a permanent of that type on the
  battlefield, and at `PreModifier` the type alone.
- `NounWord` is gone, and no description in `plugins/` is a flat `and([…])` of `Predicate`s; the
  sorts the answers settle on exist, each declaration with a real Vintage-legal caller.
- The corpus is respelled by a script, with the script and an old-to-new table in `docs/evidence/`,
  each site checked against Oracle text or rule text. At a type word before a "card" the term
  dropped, the respell writes the printed "card" (the survey's sites: *Abolish*, *Ashes of the
  Fallen*, *Bleeding Effect*, *Bloodfire Enforcers*, *Cairn Wanderer*, *Carrion Grub*, *Celebrate
  the Harvest*, *Charnel Troll*, *Crypt Lurker*, *Cultivator Colossus*, *Custodi Squire*,
  *Death-Mask Duplicant*, *Death or Glory*, *Delivery Moogle*, *Garruk Relentless // Garruk, the
  Veil-Cursed*, *Hymn of Rebirth*, *Land Tax*, *Midnight Scavengers // Chittering Host*, *Mox
  Diamond*, *Old-Growth Dryads*, *Phyrexian Ingester*, *Rogues' Gallery*, *Scapeshift*, *Sphinx
  of the Chimes*, *Syr Konrad, the Grim*, *Thawing Glaciers*, *Urborg Scavengers*, *Voracious
  Brood*, *Collected Company*, *Commune with the Gods*, *Tezzeret's Gatebreaker*, *Skill
  Borrower*, *Conspicuous Snoop*), since a type word there is a premodifier and as a head would
  change the meaning; and it drops an unprinted card type beside a subtype (*Crash*, *Gush*,
  *Heart of Yavimaya*, *Invigorate*, *Kopala, Warden of Waves*, *Kyren Legate*, *Massacre*, *Mogg
  Salvage*, *Rouse*, *Skeleton Ship*, *Snuff Out*, *Thwart*, *Thranduil's Company*) and
  *Encroaching Mycosynth*'s five card types for "permanent spells" [CR#110.4b]. Every other site
  means what it meant, and the script reports, per site, the tree before and after.
- The landing runs the `syl-number-snapshot` diff and explains every flagged line.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check`, `cargo test -p xtask` and
  `cargo clippy -p xtask --tests -- -D warnings` pass; `cargo xtask cite check
  --list-noncompliant` is empty and `cargo xtask cite check` reports 0 stale.

## Not in scope

- The fits relation between head nouns that anaphors read ("that player" reading an opponent), and
  encore's "that opponent": `syl-opponent-noun-and-predicate`.
- Created tokens and characteristic-setting complements: `syl-token-records-to-templates`,
  `syl-become-replaces-types`.
- The heads `damage`, `life` and `counter` and a counter kind standing at `Nominal`:
  `syl-damage-and-life-noun-phrases`, `syl-counters-noun-phrase`.
- Repeated determiners over one shared nominal: *Balthor the Defiled*'s "Each player returns all
  black and all red creature cards from their graveyard to the battlefield". The English grammar
  has no Reading for it yet (it misreads "all black" as a noun phrase of its own):
  `english-shared-nominal-determiner-coordination`. Its term waits on that Reading; it is a
  Template or a declaration of its own, not the `shared` of `syl-shared-phrase-written-once`.
- Location postmodifiers ("in your graveyard", "exiled with …") beyond the retype:
  `syl-zone-words`.

## Pointers

- `docs/design/spelling.md` (substitution, optional and list Holes, the surface term as the
  derivation, the notation, rejected alternatives); `docs/design/syl.md` §2, §3, §5;
  `docs/design/semantics.md` §2.4, §3.1
- `plugins/builtin/coercions/`, `plugins/builtin/primitives/noun/`,
  `plugins/builtin/primitives/nounWord/`, `plugins/builtin/primitives/predicate/`,
  `plugins/builtin/templates/predicate/`, `plugins/builtin/rules/`,
  `crates/xtask/src/dsl_check/{schema,coercion}.rs`, `crates/core/src/filter.rs`
- `crates/english/grammar/english.fug` (`Determined`, `AttributedNominal`, the Predicative and
  Quality productions)
- Evidence: `docs/evidence/syl-type-word-templates/`,
  `docs/evidence/cleanup-step6-syl/head-noun-study.md`,
  `docs/evidence/cleanup-step6-syl/english-vs-syl-audit-verification.md` (§1)
