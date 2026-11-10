---
needs: [cleanup-step6-spine]
---

# Type words are spelled only through Templates that take them

## Why

A description that includes a card type or subtype, refers to no zone and has none of the words
"card", "spell" or "source" means a permanent of that type on the battlefield [CR#109.2]; "card"
with a zone, "spell" and "source" give it other domains [CR#109.2a,109.2b,109.2c]. Nothing in a
term says so. `isCreature` is a Template spelled "creature" over `hasType(creature)`, the eight
other single-type Templates are alike, and `hasType`, `hasSubtype` and `hasSupertype` are each
spelled by their argument, so "creature" has two written forms and neither carries the
battlefield.

A survey of the checker's own resolution (every call `cargo xtask dsl check` resolves, traced)
found 1,159 sites where a type, subtype or supertype word reaches a `Predicate`: 963 in canon
cards, 196 in builtin bodies, 9 of them the single-type Templates' own bodies. By the printed
construction each stands in, checked against Oracle text (cards) and rule text (bodies):

| | canon | builtin |
|---|---|---|
| a. the description's head, permanent reading: "target creature", "each Goblin" | 524 | 83 |
| b. before "card", "spell", "token", "source", a zone or another type word; after "non" | 170 | 29 |
| c. coordinated: "artifact or enchantment", "instant or sorcery card" | 79 | 34 |
| d. complement: "If it's a land", "is a Human" | 11 | 14 |
| e. keyword argument: `enchant` 84, affinity, protection, splice, equip, devour, empower | 83 | 9 |
| f. object of `play` (12), departed object ("the sacrificed creature") | 15 | 3 |
| g. supertype: "legendary", "basic land", "Nonbasic lands", "snow permanents" | 20 | 14 |
| h. "any target" (39 sites print no type word) and type words the card does not print (22) | 61 | 1 |

37 of the class-a sites are the subject of a zone-change event ("Whenever a creature dies", "If a
creature would enter"). Class f and those 37 name objects that are not on the battlefield when
the text is read. The 22 unprinted type words are a card type beside a subtype (16 sites on 12
cards, as *Gush*'s `isLand` in "two Islands you control"), *Thranduil's Company*'s `isCreature`
for "another Elf", and *Encroaching Mycosynth*'s "permanent spells" written as five card types
[CR#110.4b].

## Design

The written forms below use `spelling.md` §7's notation (`$type card`); the corpus still writes
`<type> card`.

- **The bare type tests have no spelling.** `hasType`, `hasSubtype` and `hasSupertype` drop their
  `spelling`, and so does any other declaration whose wording is a bare type, subtype or supertype
  Hole over a type test. Builtin bodies still call them unspelled. A type word is spelled only
  through a Template that takes it.
- **One Template per printed construction, taking the type word as its argument**, never one
  Template per type. A card type and a subtype are different sorts (`CardType`, `Subtype`
  [CR#205.2a,205.3a]); the checker resolves an argument at its parameter's one declared sort and has
  no union sorts, so each construction that admits both is two Templates, one per sort. One Template
  over both would need a new sort, two coercions into it and a primitive behind each coercion,
  coupling two CR concepts for the sake of a shorter list.
- **The head reading**: `$type`, body `and([hasType($type), isPermanent])`, and `$subtype`, body
  `and([hasSubtype($subtype), isPermanent])` [CR#109.2]. *Cloudshift* ("Exile target creature you
  control"); *Adelbert Steiner* ("for each Equipment you control"). `enchant` and affinity take a
  head-reading description, since enchant names its object [CR#702.5a] and affinity counts "each
  [text] you control" [CR#702.41a]: *Arcane Flight* ("Enchant creature"), *Frogmite* ("Affinity for
  artifacts").
- **Supertypes are never a head.** `$supertype`, body `hasSupertype($supertype)`, stands only beside
  a head; no `Supertype` word is also a `CardType` or `Subtype` word, so its wording is nobody
  else's. *Plaza of Heroes* ("Target legendary creature", "a legendary spell"); *Skred* ("snow
  permanents you control"); *Thawing Glaciers* ("a basic land card"). Its negation follows Q1:
  *Blood Moon* ("Nonbasic lands").
- **The nine single-type Templates are deleted**, with every call site respelled to the Template of
  its printed construction. `isHistoric` ("historic" [CR#700.6]) is always a premodifier and keeps
  its own wording over unspelled tests. "Outlaw" is a head ("outlaws you control", permanents
  [CR#700.12a], *At Knifepoint*) and a premodifier ("each outlaw creature", *Caught in the
  Crossfire*), so `isOutlaw` splits as a subtype word does under the first open question.
  `isPermanentType` stays an unspelled helper and `isPermanentCard` stays "permanent card"
  [CR#110.4a]; "permanent spell" gets its Template [CR#110.4b] (*Encroaching Mycosynth*).
  `isInstantOrSorcery` becomes the coordination the first open question settles. `isAnyTarget`'s
  body names creatures, planeswalkers and battles through the head reading [CR#115.4], which it
  lacks today.

## Open questions

- **Q1. How a type word that is not the head is written.** Classes b to e, less `enchant`.
  - (A) A Template pair per construction, as `$type card` is: `$type card` / `$subtype card`, body
    `and([hasType($type), isCard])` (*Necromancy*; *Abolish*'s "a Plains card"); `$type spell` /
    `$subtype spell` (*Feroz's Ban*; *Daru Warchief*'s "Soldier spells"); `$type token` / `$subtype
    token`, which [CR#109.2] makes permanents, body `and([hasType($type), isToken, isPermanent])`
    (*Anointer Priest*'s "creature token you control"; *Eternal Skylord*'s "Zombie tokens you
    control"); `$type source` / `$subtype source` [CR#109.2c] (*Argothian Treefolk*'s "artifact
    sources"; *Woodland Weavemaster*'s "Elf sources"); `non~$type` / `non~$subtype` /
    `non~$supertype` (*Nekrataal*; *Grumgully, the Generous*'s "non-Human"; *Blood Moon*);
    complements `$subject is $type` / `$subject is $subtype` beside `is` (*Prophecy*'s "If it's a
    land"; *Harvest Hand*'s "is a Human"); a stacked premodifier that takes what it modifies, `$type
    $p` / `$subtype $p`, body `and([hasType($type), $p])` (*Saheeli, Filigree Master*'s "Artifact
    creatures", *Haven of the Spirit Dragon*'s "Dragon creature spell", *Mishra's Factory*'s
    "Assembly-Worker creature"); coordinated type words before a carrier as a list Hole,
    `$types<$coordinator> card` and the like, over a new list-taking test primitive per sort
    (*Commune with the Gods*, *Custodi Squire*, *Flooded Strand*'s "Plains or Island card"), each
    list Template refusing one element, which would print as its single-word twin; and a twin of
    each keyword whose quality is a type word, one per sort it admits: protection [CR#702.16a]
    (*Angelic Curator*'s "protection from artifacts", *Baneslayer Angel*'s "from Demons and from
    Dragons"), splice [CR#702.47a] (*Candles' Glow*), equip [CR#702.6c] (*Steelclaw Lance*'s "Equip
    Knight", *Blackblade Reforged*'s "Equip legendary creature"), devour [CR#702.82c], landwalk
    [CR#702.14c] (*Lord of Atlantis*), typecycling [CR#702.29e] (*Absorb Vis*'s "Basic
    landcycling"). About 40 declarations, each needing a real caller, and mixed coordination stays
    unwritable: *Tezzeret's Gatebreaker*'s "a blue or artifact card", *Soldevi Adnate*'s "a black or
    artifact creature".
  - (B) One more Template per sort: the test reading `$type` / `$subtype`, body the bare test, which
    never heads a description. Every non-head type word is a flat conjunct as today, and the generic
    `non` takes the test and supertype readings ("Nonbasic" is `non(<supertype>(basic))`): "creature
    card" is `and([<test>(creature), isCard])`, "artifact creature" `and([<test>(artifact),
    <head>(creature)])`, "nonland" `non(<test>(land))`, "If it's a land" `is(it, <test>(land))`,
    "protection from enchantments" `protection(<test>(enchantment))` (*Tattoo Ward*), "a blue or
    artifact card" `and([or([hasColor(blue), <test>(artifact)]), isCard])`. `card`, `spell`,
    `token`, `source` and `permanent` stay the heads they are. Five Templates with the head and
    supertype readings, no keyword twins, no list primitives; the head and test Templates share the
    wording `$type` at `Predicate`, told apart only by where they stand (Q2).
  - Recommendation: (B). Under (A) Q2's rule is still needed (every pair it lists exists under (A)),
    so (A) pays for forty declarations and a coordination gap without removing the one same-wording
    pair (B) states openly.
- **Q2. Where a head stands.** Under either answer, any `Predicate` Hole can hold the head reading
  where the text means the bare type, so the same printed words keep a second, wrong term. The pairs
  this design leaves, each at one sort:
  - `is(s, <head>(land))` against the complement reading (A: `$subject is $type`; B: `is(s,
    <test>(land))`), at `Condition`: *Prophecy*.
  - `non(<head>(land))` against `non~$type` (A) or `non(<test>(land))` (B), at `Predicate`; the
    generic `non` stays for colors (7 sites), "nonmana" and "non-outlaw".
  - A quality parameter holding the head reading against the quality reading, at `Ability`:
    protection, splice, equip, devour, landwalk, typecycling, hexproof from.
  - `and([<head>(creature), isCard])` against `$type card` (A) or `and([<test>(creature), isCard])`
    (B), at `Predicate`, and the same with `isSpell`, `isToken`, `isSource`: two heads in one
    description, the first wrongly on the battlefield.
  - `and([<head>(artifact), <head>(creature)])` against the stacked form, at `Predicate`; same
    meaning, two written forms.
  - Under (B) only: `<head>` and `<test>` themselves.

  Every pair has one cause: nothing says which Hole is a description's head. The fix is a rule that
  the head reading (with `card`, `spell`, `token`, `source`, `permanent`, `player`) stands only as a
  description's head, at most one per description, coordinated heads counted as one, and that the
  test reading never does. Options: (a) a checker rule over the Holes that are description heads (a
  determiner's `p`, `enchant`'s subject, affinity's quality), marked on the parameter; (b) a sort
  for descriptions with a head, `syl-description-head-noun`'s D1 and D2. Recommendation: (a). The
  head-reading Templates are themselves the head marker that ticket found too costly to write, and
  no term changes shape.
- **Q3. What `isCard` means.** A card is a Magic card or an object represented by one [CR#108.2]; a
  token is not [CR#108.2b]; a permanent is a card or token on the battlefield [CR#110.1], so a
  nontoken permanent is a card; a "card" description with a zone means a card in that zone
  [CR#109.2a]. Core's `ObjectClass::Card` says the same (`crates/core/src/filter.rs`).
  `not(or([in(battlefield), in(stack)]))` would contradict [CR#110.1] and make *Encroaching
  Mycosynth*'s printed "nonland permanent cards you own that aren't on the battlefield" redundant.
  Where no zone is printed, the verb supplies it ("search your library for a creature card",
  "discard a creature card"). Recommendation: `isCard` is the class alone, never a zone; the zone is
  what the text prints, in the description or in its verb.
- **Q4. `isPermanent` or `in(battlefield)` in the head reading.** Under the rules they are one set:
  a permanent is a card or token on the battlefield [CR#110.1], and every object on the battlefield
  is a permanent [CR#403.3]. In `.syl` they are one tree, since `isPermanent`'s body is
  `in(battlefield)`. Core keeps `Class(Permanent)` apart from `InZone(Battlefield)`, and the engine
  computes the class from the zone (`crates/engine/src/target.rs`). Recommendation: `isPermanent`,
  the word [CR#109.2] uses ("a permanent of that card type"), so `in(battlefield)` stays the written
  form of a printed "on the battlefield", as `syl-card-head-noun-sweep` writes it.
- **Q5. The object of `play`, and departed objects.** "Play lands" (*Exploration*, *Courser of
  Kruphix*, *Crucible of Worlds*) names land cards [CR#116.2a,305.1], not permanents; "the
  sacrificed creature" (*Soldevi Adnate*), "creatures destroyed this way" (*Phyrexian Rebirth*) and
  the 37 zone-change subjects name objects off the battlefield. A leaves-the-battlefield trigger
  looks back [CR#603.10a], an entering permanent is checked as it would exist on the battlefield
  [CR#614.12], and a departed object's information is its last known [CR#608.2h]. Options: (a) keep
  the head reading and read each such description at its event's time: before for departed objects,
  as it would exist after for `play` and "would enter", which `semantics.md` §4.5 states and a
  lowering test proves; (b) a card-reading Template for `play`'s object. Recommendation: (a). It is
  the reading the 37 trigger subjects need already, and (b) is one more same-wording pair under Q2.
- **Q6. Names.** Recommendation: the head reading `typed` / `subtyped` (`target(typed(creature))`);
  under (B) the test reading `ofType` / `ofSubtype`, the supertype `supertyped`, and the generic
  `non` kept; under (A) `typeCard`, `typeSpell`, `typeToken`, `typeSource`, `nonType`, `isType`,
  `typeBefore` and their `subtype` twins. `ofType` is taken at `NounWord`
  (`plugins/builtin/primitives/noun/`), so under (B) either that primitive is renamed or the test
  reading takes another name.

## Done when

- Q1 to Q6 are answered, and each answer is written where it governs: the Templates' doc comments,
  `spelling.md` for Q2, `semantics.md` §4.5 for Q5.
- `docs/decisions/semantics.md` states [CR#109.2]: a description with a card type or subtype, no
  zone and none of "card", "spell" or "source" means a permanent of that type on the battlefield,
  and the head-reading Templates are where a term says so.
- The Templates the answers settle on exist, each with a real Vintage-legal caller (a canon card,
  authored if it is not yet in `plugins/canon/cards/`), at least the cards this ticket names for it.
- No declaration's `spelling` is a bare type, subtype or supertype Hole over a bare type test, and
  the nine single-type Templates are gone.
- The corpus is respelled by a script, with the script and an old-to-new table in `docs/evidence/`.
  It covers every site the checker trace finds at the time (the survey found 1,159), each written as
  the Template of its printed construction and checked against Oracle text or rule text. That
  includes the type-word sites `syl-card-head-noun-sweep` listed: a type word before a dropped
  "card" (*Abolish*, *Ashes of the Fallen*, *Bleeding Effect*, *Bloodfire Enforcers*, *Cairn
  Wanderer*, *Carrion Grub*, *Celebrate the Harvest*, *Charnel Troll*, *Crypt Lurker*, *Cultivator
  Colossus*, *Custodi Squire*, *Death-Mask Duplicant*, *Death or Glory*, *Delivery Moogle*, *Garruk
  Relentless // Garruk, the Veil-Cursed*, *Hymn of Rebirth*, *Land Tax*, *Midnight Scavengers //
  Chittering Host*, *Mox Diamond*, *Old-Growth Dryads*, *Phyrexian Ingester*, *Rogues' Gallery*,
  *Scapeshift*, *Sphinx of the Chimes*, *Syr Konrad, the Grim*, *Thawing Glaciers*, *Urborg
  Scavengers*, *Voracious Brood*, *Collected Company*, *Commune with the Gods*, *Tezzeret's
  Gatebreaker*, *Skill Borrower*, *Conspicuous Snoop*), and an unprinted card type beside a subtype,
  dropped (*Crash*, *Gush*, *Heart of Yavimaya*, *Invigorate*, *Kopala, Warden of Waves*, *Kyren
  Legate*, *Massacre*, *Mogg Salvage*, *Rouse*, *Skeleton Ship*, *Snuff Out*, *Thwart*, *Thranduil's
  Company*), and *Encroaching Mycosynth*'s "permanent spells".
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Not in scope

- A type word as a head noun at `NounWord` (`that(creature)`, `enchanted(creature)`; 252 sites), and
  "a creature type" at `QualitySort`: neither reaches a `Predicate`.
- Type words in characteristics (`becomes a 3/3 Angel creature`, token definitions).
- Heads that are not type words ("permanent", "card" after a color): `syl-card-head-noun-sweep`.

## Pointers

- `docs/decisions/spelling.md` §2, §3.1, §4, §7; `docs/decisions/semantics.md` §2.4, §4.5;
  `docs/decisions/syl.md` §2, §3
- `plugins/builtin/primitives/predicate/{hasType,hasSubtype,hasSupertype,isCard}.syl`,
  `plugins/builtin/templates/predicate/`, `plugins/builtin/coercions/`,
  `crates/xtask/src/dsl_check.rs`, `crates/core/src/filter.rs`
- `syl-description-head-noun` (the structural alternative), `syl-card-head-noun-sweep`,
  `syl-opponent-noun-and-predicate`
- Survey: `docs/evidence/syl-type-word-templates/` (`summary.txt`, `sites.tsv`, `survey.py`,
  `checker-trace.patch`)
