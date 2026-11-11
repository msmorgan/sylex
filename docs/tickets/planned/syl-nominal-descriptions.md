---
needs: [cleanup-step6-spine, syl-spelling-optional-segments]
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

- A description is a `Nominal`: its head, with a `List(PreModifier)` before it and a
  `List(PostModifier)` after it, built by `nominal(pre = [], head, post = [])`. Each list's
  elements stand in printed order. `headed` is taken by an existing primitive ("$subject as a
  $head"), so the constructor is `nominal`.
- `pre` is declared before the required `head` because it is printed first, and arguments are
  written in declaration order (`syl.md` §2).
- A copula's complement is a `Predicative`: what `is(subject, …)` takes ("If it's a land",
  "is tapped", "is in your graveyard").
- A type word coerces to two things, and the sort of the slot it stands in decides which: at
  `Nominal` it is a permanent of that type on the battlefield [CR#109.2]; at `PreModifier` it is
  the bare type test. No `Predicate` coercion infers a reading from its context.
- `NounWord` is merged into `Nominal`: every `NounWord` consumer takes a `Nominal`, and each head
  word means at `Nominal` what today's description word of the same name means (`opponent` what
  `isOpponent` means). `ofType` and `copied` become a `nominal` with a premodifier.
- `ConferralRule.scope` and `SbaRule.scope` take a `Nominal`.

## Open questions

- **Words printed both before the head and after it or as a complement**: `tapped` ("tapped
  creature", "it's tapped"), `white`, `attacking` ("attacking creature", "creatures attacking
  you"). A coercion from `Status` or `Color` into `PreModifier` reaches the bare word only at that
  sort; the `PostModifier` and `Predicative` forms need a declaration each, and a `PreModifier`
  Template named `tapped` would collide with the `Status` word `tapped` under Reaches(`PreModifier`)
  (`syl.md` §3). Options: a coercion into each of the three sorts from `Status` and `Color`; or a
  coercion into one and declarations at the others under other names. Recommendation: coercions
  into `PreModifier` and `Predicative`, which print the bare word in both places, and
  `PostModifier` declarations only where a word takes a complement ("attacking you").
- **Whether `DesignationLabel` coerces into `PreModifier` wholesale.** None of the six source sorts
  (`CardType` 10, `Subtype` 463, `Supertype` 4, `Color` 5, `Status` 7, `DesignationLabel` 21)
  shares a name with another, so no coercion collides. But `DesignationLabel` holds non-adjectival
  labels (`monarch`, `initiative`, `day`, `night`, `level`, `commander`, `citysBlessing`), which a
  wholesale coercion admits as premodifiers. Recommendation: no coercion from
  `DesignationLabel`; the labels printed as premodifiers ("monstrous", "renowned", "goaded") get a
  `PreModifier` declaration each.
- **Coordinated heads.** "Target white creature or black Zombie you control" coordinates two
  nominals under one postmodifier: `nominal(or([nominal(pre = [white], creature),
  nominal(pre = [black], zombie)]), post = [youControl])`. Whether `head` takes a coordination of
  nominals, and whether `nominal(pre = [a], nominal(pre = [b], c))` is refused in favor of
  `nominal(pre = [a, b], c)` so a nominal has one tree. A coordination at `Nominal` is a union
  ("all artifacts and creatures"), where `pre = [artifact]` on `creature` is an intersection.
  Recommendation: both, with the refusal a checker rule and a negative test.
- **Stacked premodifiers inside a premodifier coordination.** "Artifact or non-Aura enchantment
  card", "Knight, Aura, Equipment, or legendary artifact card": one arm of an `or` at `pre` is
  itself several premodifiers (`NounPremodifierConjunct`, `crates/english/grammar/english.fug`). Whether a
  `PreModifier` coordination's arm may be a list of premodifiers, and by what constructor.
  Recommendation: yes, through one constructor at `PreModifier` that takes a list printed with no
  join, refused with one element.
- **A group property printed as a postmodifier.** *Eerie Ultimatum* prints "Return any number of
  permanent cards with different names from your graveyard to the battlefield", and writes
  `withDifferentNames(counted(anyNumber, and([isPermanentCard, in(yourGraveyard)])))`: the
  wrapper stands outside the determiner, so it prints "with different names" after "from your
  graveyard". `withDifferentNames` and `withTheSameName` (7 files call them) are
  `NounPhrase -> NounPhrase` over `agree`, an Agreement that holds of the group, not of each member
  (glossary **Agreement**), so a `PostModifier` in the member list would test each card alone.
  Options: a `PostModifier` whose meaning the determiner lifts to the group it forms; or a
  Template at `NounPhrase` taking the determiner's quantity and the nominal, with the group words
  printed between the nominal's postmodifiers. Recommendation: the second, which keeps the
  Agreement on the group and the words in printed order.
- **One operator printed on both sides of the head.** `nominal(pre, head, post)` gives each
  modifier one side. Two constructions put one operator's words on both:
  - an ordinal over a head and its relative: *Highspire Bell-Ringer*'s "The second spell you cast
    each turn costs {1} less to cast", which ranks the spells cast each turn, and is written today
    with the ordinal inside the lookback (`did(…, ordinal = {ordinal = nth(2), per = turn})`);
  - a comparative premodifier with its "than" complement after the head
    (`ComparativeIndirectComplement`, `crates/english/grammar/english.fug`): *Conformer Shuriken*'s granted
    "If that creature has greater power than this creature", *Death's Oasis*'s "a creature card
    with lesser mana value than the creature that died".

  Options: a Template at `Nominal` per construction whose wording has fixed words and Holes on
  both sides of the head Hole ("the $ordinal $head $relative"), which is substitution, as any
  wording with fixed words on both sides of its Holes is; or a premodifier and a postmodifier that
  the checker pairs. Recommendation: the Templates, one per construction, each with its card as
  caller.
- **Two well-sorted trees for one nominal.** "Artifact or enchantment you control" is
  `nominal(or([artifact, enchantment]), post = [youControl])` or
  `or([artifact, nominal(enchantment, post = [youControl])])`, and nothing in the ADRs picks
  between well-sorted candidates (`spelling.md`, on what spelling leaves to meaning). Recommendation:
  the owner rules which attachment canon writes, and `spelling.md` says how recovery picks.
- **Consumers that are not descriptions.** Eight classes of declaration take a `Predicate` for a
  phrase that is no description:
  1. spend purposes: `toCast`, `toTurnFaceUp`, `toActivate` (`syl-tocast-noun-phrase`);
  2. the copula `is` ("is a creature", "is tapped", "is in your graveyard", "it's 4/3", "is tapped
     and attacking");
  3. `transform` and `turnOver`'s result (*Cult of the Waxing Moon*'s "into a non-Human creature",
     *Afflicted Deserter*'s "into Werewolf Ransacker");
  4. `procedure`'s `resulting`, which holds both a printed depictive (siege's "cast it
     transformed") and unprinted rule filters (`discover`, `cascade`, `flashback`, `jumpStart`);
  5. fifteen keyword qualities: `protection` and `hexproofFrom` ("from artifacts", "from red"),
     `affinity` ("for artifacts"), `champion` and `behold` ("a Merfolk", "a Dragon"), `enchant`,
     `equipQuality`, `emergeFrom`, `devourQuality`, `bandsWithOther`, `offering`, `landwalk`,
     `typecycling`, `splice`, and `partnerWith`, which names a card;
  6. exceptions and holders: `allExcept`, `abilityGrantFrom` and `allAbilityLoss` ("except mana
     abilities"), `keywordGrant.holders` (*Cairn Wanderer*'s "a creature card with flying is in a
     graveyard"), `heldFor` ("the world supertype");
  7. `chosenQuality` ("the chosen type", which *Arcane Adaptation* writes with a nested
     `ofChosen(subtype(creature))`);
  8. `aCharacteristic`, a finite verb phrase ("has mana value 2 or less", "shares a card type"),
     with no call site in `plugins/`.

  Also taking `Predicate`, not in the eight: `described`, `nameOfCard`, `players`, `withQuality`,
  `selectRandom`, `spendOnlyToCastAnd`, `devourEntry`, `aggregate`; and `someOf` takes a
  `NounPhrase`. Recommendation: this ticket retypes every description consumer and class 2
  (`Predicative`); classes 1, 3, 4, 5, 6, 7 and 8 keep `Predicate` until a ticket gives each its
  sort (class 1 is `syl-tocast-noun-phrase`'s), and the landing lists what still takes
  `Predicate` and why.
- **Rules scopes written as conjunctions.** Four of the eight rules rows scope by an `and([…])`
  that needs respelling as a nominal, not only a retype: `rules/conferral/toxicPoison.syl`
  ("creature with toxic"), `foretoldCardCast.syl`, `foretoldCardLook.syl` and
  `plottedCardCast.syl` (a card with a designation in exile). Recommendation: respell them as
  nominals with the head the rule's text names, in this ticket.
- **A complement with a type word** ("If it's a land", *Prophecy*, of a card in a library; "If the
  exiled card is a snow land", *Storm Elemental*). A `Predicative` that takes a `Nominal` gives the
  type word the permanent reading [CR#109.2], which these cards do not mean. Options: the
  `Predicative` "a $x" takes a `PreModifier`-headed form with no permanent reading; or it takes a
  `Nominal` read at the subject's zone. Recommendation: the first.
- **What the head `card` means.** A card is a Magic card or an object represented by one
  [CR#108.2]; a token is not [CR#108.2b]; with a zone, a card in that zone [CR#109.2a].
  Recommendation: the class alone, never a zone; the zone is what the text prints, in the
  description or in its verb.
- **How the permanent reading is written.** Under the rules a permanent is a card or token on the
  battlefield [CR#110.1], and `isPermanent`'s body is `in(battlefield)`. A coercion's body is a
  primitive at its result sort applied to its one parameter (`syl.md` §2), so the reading lives in
  the `Nominal` primitive the coercion goes through and in lowering, not in the coercion.
  Recommendation: the primitive lowers to the type test together with the permanent class, and a
  printed "on the battlefield" stays `in(battlefield)` as a postmodifier.
- **The head `token`**: a token on the battlefield, since a token elsewhere ceases to exist
  [CR#111.7], or the token test alone. Recommendation: the token test alone, which the
  zone-change subjects ("a token you control dies") need.
- **The object of `play` and departed objects.** "Play lands" (*Exploration*, *Courser of
  Kruphix*, *Crucible of Worlds*) names land cards [CR#116.2a,305.1]; "the sacrificed creature"
  (*Soldevi Adnate*) and the 37 zone-change subjects ("Whenever a creature dies") name objects
  off the battlefield. A leaves-the-battlefield trigger looks back [CR#603.10a], an entering
  permanent is checked as it would exist on the battlefield [CR#614.12], and a departed object's
  information is its last known [CR#608.2h]. Recommendation: keep the permanent reading and read
  each such description at its event's time, which `semantics.md` §4.5 states and a lowering
  test proves.
- **Names and glossary.** `Nominal`, `PreModifier`, `PostModifier` and `Predicative` have no entry
  in either `CONTEXT.md`; the game-model glossary's **Predicate** ("A truth condition tested
  against a candidate") and **Head Noun** entries describe today's split. Recommendation: the
  four names as written, `Nominal` and `Predicative` matching the English grammar's Categories of
  those names (`english.fug`), with glossary entries, and **Predicate** kept for what still takes
  it.

## Done when

- The open questions are answered, and each answer is written where it governs: `spelling.md`,
  `syl.md`, `semantics.md` §2.4 and §4.5, and the declarations' doc comments.
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

- Restoring other dropped heads ("permanent", and "card" after a color or a zone), the "on the
  battlefield" sites written as `isPermanent`, and hexproof's headless body:
  `syl-card-head-noun-sweep`.
- The fits relation between head nouns that anaphors read ("that player" reading an opponent), and
  encore's "that opponent": `syl-opponent-noun-and-predicate`.
- Created tokens and characteristic-setting complements: `syl-token-records-to-templates`,
  `syl-become-replaces-types`.
- Repeated determiners over one shared nominal: *Balthor the Defiled*'s "Each player returns all
  black and all red creature cards from their graveyard to the battlefield". It is a coordination
  of two determined phrases that print their shared head once, which `spelling.md`'s shared material at a coordination governs:
  `syl-shared-phrase-written-once` lists it among the coordinations that take a shared phrase.
- Location postmodifiers ("in your graveyard", "exiled with …") beyond the retype:
  `syl-zone-words`.

## Pointers

- `docs/decisions/spelling.md` (substitution, optional and list Holes, the resolved term as the
  derivation, the notation, rejected alternatives); `docs/decisions/syl.md` §2, §3, §5;
  `docs/decisions/semantics.md` §2.4, §4.5
- `plugins/builtin/coercions/`, `plugins/builtin/primitives/noun/`,
  `plugins/builtin/primitives/nounWord/`, `plugins/builtin/primitives/predicate/`,
  `plugins/builtin/templates/predicate/`, `plugins/builtin/rules/`,
  `crates/xtask/src/dsl_check/{schema,coercion}.rs`, `crates/core/src/filter.rs`
- `crates/english/grammar/english.fug` (`Determined`, `AttributedNominal`, the Predicative and
  Quality productions)
- Evidence: `docs/evidence/syl-type-word-templates/`,
  `docs/evidence/cleanup-step6-syl/head-noun-study.md`,
  `docs/evidence/cleanup-step6-syl/english-vs-syl-audit-verification.md` (§1)
