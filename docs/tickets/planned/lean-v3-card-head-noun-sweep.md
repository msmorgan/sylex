---
needs: [plugins-v3-builtin-primitives-and-templates, lean-generated-canon-modules, semantics-rust-rewrite]
---
# Restore the printed head nouns that card spellings drop

Standard constraints apply. Found by the 2026-10-25 triage of an external audit ("Atraxa"),
verified against the tree at `wmyykqnv`. Fragment paths are Lean proof-suite files under
`lean/Semantics/`, which `semantics-rust-rewrite` carries to Rust tests; edit each fragment where
it lives then, finding it by its identifier (line numbers are historical, as of that change).
Whole-card declarations are `.syl` in `plugins_v3/canon/cards/`, with the current paths listed
below. Test names below are the Lean pin names.

A Head Noun narrows what a phrase is known to be (`docs/contexts/game-model/CONTEXT.md`, **Head
Noun**, [CR#109.1,205.2a]). Where Oracle prints "permanent", "card" or "creature", the spelling
writes the matching predicate. The rulings already in force: `lean-v3-authoring-surface` §3
(`docs/tickets/done/lean-v3-authoring-surface.md:552`) gives the canonical spellings
`and [isCard, in_ yourGraveyard]`, `and [isCreature, isCard, in_ yourHand]` and
`and [isCard, in_ (exiledWith thisArtifact)]`, and `lean-v3-typed-verbed-deeds` rewrote sites "to
match their Oracle text … actual head nouns". The Templates are `isPermanent`
(`plugins_v3/builtin/templates/predicate/isPermanent.syl`, `.in_ battlefield`), `isCard` (primitive,
`plugins_v3/builtin/primitives/predicate/isCard.syl`) and `cardIn`
(`plugins_v3/builtin/templates/predicate/cardIn.syl`, `and [isCard, in_ zone]`). `youControl` and
`controlledBy` (`youControl.syl`, `controlledBy.syl` in the same directory) are possessor tests only and do not imply the
battlefield, so dropping "permanent" widens the set to spells you control.

The sites using `exiledWithThisArtifact` (Muse Vessel, Synod Sanctum, Chrome Mox, Ice Cauldron,
Idol of Endurance, Cold Storage) are **not** in this ticket; `lean-v3-template-twins` §6 owns them.

## 1. Sites with no head noun at all

Rewrite each exactly as given. Every rewrite was compiled in a scratch file on 2026-10-25 and
checked `[]`.

| # | Site | Card, Oracle | Now | Write |
|---|---|---|---|---|
| 1 | `Proofs/CardsStatic.lean:473` | Seedborn Muse: "Untap all permanents you control during each other player's untap step." | `allOf youControl` | `allOf (and [isPermanent, youControl])` |
| 2 | `Proofs/CardsStatic.lean:496` | Blatant Thievery: "For each opponent, gain control of target permanent that player controls." | `target (controlledBy (that player))` | `target (and [isPermanent, controlledBy (that player)])` |
| 3 | `Proofs/CardsCounters.lean:44-47` (`alaundoTheSeer`) | Alaundo the Seer: "…Then remove a time counter from each other card you own in exile." | `each (and [youOwn, in_ …])` | `each (and [isCard, isOther, youOwn, in_ exileZone])`. This also restores the printed "other". In the fragment the Contrast Anchor falls back to the source (`docs/decisions/semantics-v3.md` §0.1 item 2, step 4); in the whole card it is the card the ability exiled first (step 2). |
| 4 | `Proofs/CardsStatic.lean:779` | Myr Welder: "This creature has all activated abilities of all cards exiled with it." | `allOf (in_ (exiledWith this_))` | `allOf (cardIn (exiledWith this_))` |
| 5 | `Proofs/CardsChoice.lean:1290` | Rogue Class level 3: "You may play cards exiled with this Class, …" | `play (allOf (in_ (exiledWith thisClass)))` | `play (allOf (cardIn (exiledWith thisClass)))`. Leave `toCast (in_ (exiledWith thisClass))` at `:1292` as it is (see "Not in this ticket"). |
| 6 | `Proofs/CardsCost.lean:680` | Void Maw: "Put a card exiled with this creature into its owner's graveyard: …" | `put (a (in_ (exiledWith thisCreature))) graveyard` | `putInto (a (cardIn (exiledWith thisCreature))) graveyard`. The verb change is `lean-v3-template-twins` §1's; make both changes here, and that ticket skips this line. |
| 7 | `Proofs/CardsKeyword.lean:262` | Drach'Nyen: "…where X is the exiled card's power." | `powerOf (the (in_ (exiledWith thisEquipment)))` | `powerOf (the (cardIn (exiledWith thisEquipment)))` |
| 8 | `Proofs/CardsAnaphora.lean:300` | Thalia's Lancers: "…search your library for a legendary card, reveal it…" | `a (and [hasSupertype .legendary])` | `a (and [isCard, hasSupertype .legendary])` |
| 9 | `Proofs/CardsTrigger.lean:385` | Aven Shrine: "…X is the number of cards in all graveyards with the same name as that spell." | `countOf (and [in_ graveyard, (shares .cardName .share (that .spell))])` | `countOf (and [isCard, in_ graveyard, shares .cardName .share (that .spell)])` |
| 10 | `Proofs/CardsAnaphora.lean:266` | Eradicate: "…for all cards with the same name as that creature…" | `allOf (shares .cardName .share (that card))` | `allOf (and [isCard, shares .cardName .share (that card)])` |
| 11 | `Proofs/CardsChoice.lean:1112` | Memoricide: "…any number of cards with that name…" | `counted anyNumber (named chosenName)` | `counted anyNumber (and [isCard, named chosenName])` |
| 12 | `Proofs/CardsChoice.lean:1317` | Vraska's Scorn: "…a card named Vraska, Scheming Gorgon…" | `a (named "Vraska, Scheming Gorgon")` | `a (and [isCard, named "Vraska, Scheming Gorgon"])` |
| 13 | `Proofs/CardsKeyword.lean:1133` | Tower Winder: "…a card named Command Tower…" | `a (named "Command Tower")` | `a (and [isCard, named "Command Tower"])` |
| 14 | `Proofs/CardsTrigger.lean:659` and `:667` | Agency Outfitter: "…a card named Magnifying Glass and/or a card named Thinking Cap…" | `a (named "Magnifying Glass")`, `a (named "Thinking Cap")` | `a (and [isCard, named "Magnifying Glass"])`, `a (and [isCard, named "Thinking Cap"])`, at both lines |
| 15 | `Proofs/CardsKeyword.lean:1657` | Korlash, Heir to Blackblade (grandeur): "Discard another card named Korlash, Heir to Blackblade: …" | `and [named "Korlash, Heir to Blackblade", isOther, in_ yourHand]` | `and [isCard, named "Korlash, Heir to Blackblade", isOther, in_ yourHand]` |
| 16 | `Proofs/CardsCost.lean:587` | Force of Will: "…exile a blue card from your hand" | `and [hasColor .blue, in_ yourHand]` | `and [isCard, hasColor .blue, in_ yourHand]` |
| 17 | `Proofs/CardsMana.lean:453`, `:468`, `:479`, `:489`, `:500` | The five Shoals print "…exile a <color> card with mana value X from your hand" | `and [hasColor .<c>, hasManaValue .eq (letter .x), …]` | insert `isCard` as the first conjunct at each of the five lines |
| 18 | `Proofs/CardsStatic.lean:560` | Sunscour: "…exiling two white cards from your hand" | `and [hasColor .white, in_ yourHand]` | `and [isCard, hasColor .white, in_ yourHand]` |
| 19 | `Proofs/CardsKeyword.lean:1038` | Fumiko the Lowblood: "…X is the number of attacking creatures." | `countOf isAttacking` | `countOf (and [isCreature, isAttacking])` |
| 20 | `Proofs/CardsChoice.lean:471` | Rune Snag: "…each card named Rune Snag in each graveyard" | `and [named "Rune Snag", in_ graveyard]` | `and [isCard, named "Rune Snag", in_ graveyard]` |

Row 17's lines are, in order, Shining Shoal (`cardNamed "Shining Shoal"`), Disrupting Shoal, Blazing Shoal,
Sickening Shoal and Nourishing Shoal (`Proofs/CardsMana.lean:447-500`).

## 2. Sites that write the type but not "card"

Each def below prints "<type> card(s)" (Oracle checked, 2026-10-25) and writes the type
predicate with a zone but without `isCard`. In every Predicate conjunction in the def that
describes a printed "… card", insert `isCard` immediately after the last card-type, subtype or
color predicate, giving the canonical `and [isCreature, isCard, in_ yourHand]` order. Where the
conjunction is exactly `and [<zone test>]` plus types, the result is the same; do not otherwise
reorder or rewrite. Where a def has two such conjunctions (the lines are listed), change both.

- `Proofs/CardsAnaphora.lean`: `bondOfRevival` :52, `riseFromTheGrave` :233, `everAfter` :241,
  `sylvanScrying` :165, `mysticalTutor` :288, `cardNamed "Celebrate the Harvest"` :408, `cardNamed "Scapeshift"` :513
- `Proofs/CardsChoice.lean`: `continueSpell` :221, `zimoneQuandrixProdigy` :346, `preeminentCaptain`
  :352, `phyrexianIngesterPump` :410, `kohChooser` :521, `cardNamed "Ashes of the Fallen"` :892,
  `cardNamed "Cultivator Colossus"` :1085, `zimoneAndDina` :1093, `cardNamed "Crypt Lurker"` :1104, `lostHoursPlacement`
  :1120, `boreasChargerSpell` :1193 ("Plains cards"), `nautiloidShipTrigger` :1276,
  `summonEsperValigarmandaCast` :1281 and :1285, `cardNamed "Old-Growth Dryads"` :1329
- `Proofs/CardsCopy.lean`: `flawlessForgeryLine` :57
- `Proofs/CardsCost.lean`: `sistersOfStoneDeathRecall` :69, `cardNamed "Sphinx of the Chimes"` :428,
  `cardNamed "Death-Mask Duplicant"` :784, :787 and :788
- `Proofs/CardsCounters.lean`: `jhoiraOfTheGhitu` :40, `cardNamed "Voracious Brood"` :262, `cardNamed "Urborg Scavengers"` :809
  and :810, `cardNamed "Charnel Troll"` :1143 and :1149
- `Proofs/CardsDamage.lean`: `bullseyeModalCost` :995, `sandStranglerDamage` :1030, `cardNamed "Syr Konrad, the Grim"`
  :1298, `theLastRoninII` :1449
- `Proofs/CardsDescription.lean`: `cardNamed "Rogues' Gallery"` :338
- `Proofs/CardsFaces.lean`: `cardNamed "Midnight Scavengers // Chittering Host"` :451, `cardNamed "Garruk Relentless // Garruk, the Veil-Cursed"` :620 and :627
- `Proofs/CardsKeyword.lean`: `cardNamed "Bloodfire Enforcers"` :779 and :780, `stormOfSouls` :785,
  `sedrisTheTraitorKing` :835, `grixis` :841, `dralnuLichLord` :847, `cardNamed "Cairn Wanderer"` :1257 and
  :1258, `arcanumWingsAuraSwap` :1642
- `Proofs/CardsMana.lean`: `nahiriLoyaltyRead` :623, `birthingPodSearch` :701,
  `hibernationsEndTrigger` :713, `cardNamed "Delivery Moogle"` :821, `cardNamed "Mox Diamond"` :845
- `Proofs/CardsPiles.lean`: `cardNamed "Custodi Squire"` :121, `cardNamed "Death or Glory"` :158
- `Proofs/CardsStatic.lean`: `hymnOfRebirth` :38, `cardNamed "Abolish"` :543, `lhurgoyfDefinition` :649
- `Proofs/CardsTrigger.lean`: `cardNamed "Carrion Grub"` :162
- `Proofs/CardsTurn.lean`: `throughTheBreach` :25, `cardNamed "Bleeding Effect"` :297 and :299, `cardNamed "Land Tax"` :400,
  `cardNamed "Thawing Glaciers"` :462

`cardIn (exiledWith …)`-style conjunctions that already carry `isCard` (for example
`Proofs/CardsTurn.lean:604`, `Proofs/CardsTrigger.lean:632`) are already right; leave them.

`nahiriLoyaltyRead` (`Proofs/CardsMana.lean:622-624`, Nahiri, the Unforgiving's 0: "Exile target
creature or Equipment card with mana value less than Nahiri's loyalty from your graveyard") also
drops "or Equipment". Write `and [or [isCreature, hasSubtype equipment], isCard, in_ yourGraveyard,
hasManaValue .less (statOf (.stat .loyalty) this_)]`, and correct its docstring's quotation to the
printed "creature or Equipment card".

## 3. A fixture that is no card

`Proofs/CardsDamage.lean:646-651`, `firesongJoinEcho` and its pin `okFiresongJoinEcho` (`:651`), adds
"deals 1 damage to that creature or player" to Firesong and Sunspeaker, whose Oracle text is
"Whenever a white instant or sorcery spell causes you to gain life, Firesong and Sunspeaker deals
3 damage to target creature or player." Every fixture must be a Vintage-legal card's text (owner,
2026-10-25). Retire both, because the fixture asserts text no card prints. The real card stays
pinned by `okFiresongAndSunspeaker` (`Proofs/CardsKeyword.lean:166`). If `thatJoin` then has no
"that creature or player" witness, do not add one.

## What must not change

- No Template, Definition, Core constructor or `discourse` code changes. This ticket edits canon
  card declarations and the carried-over fragment tests only.
- Do not change a site's determiner, quantity, zone or any other conjunct beyond what is listed.
- No test may be weakened. Every spelled card and `ok…` test named or implied above must stay
  green. If a head-noun insertion makes a pin fail (for example a later "that card" or "it" now
  resolves differently), STOP and report the site, the pin and the refusal; do not drop the
  head noun to make it pass.

## Tests

- Rewritten in place by the edits (their trees change, their claims stay): `okAlaundoTheSeer`
  (`Proofs/CardsCounters.lean:48`), `okMyrWelderBorrowedAbilities` (`Proofs/CardsStatic.lean:780`),
  `okRogueClassLevelThree` (`Proofs/CardsChoice.lean:1294`), `okVoidMawPutCost` (`Proofs/CardsCost.lean:682`),
  `okThaliasLancersSearch` (`Proofs/CardsAnaphora.lean:302`), `okEradicateSearch` (`:269`),
  `okMemoricideSearch` (`Proofs/CardsChoice.lean:1115`), `memoricideExilesWhatItFinds`
  (`Proofs/CardFidelity.lean:234`), `okAgencyOutfitterSearch` (`Proofs/CardsTrigger.lean:662`),
  `okGrandeurDiscardCost` (`Proofs/CardsKeyword.lean:1658`), `okFumikoBushidoX` (`:1039`), and each
  §2 def's co-located `ok<Name>` pin. Also keep green the pins that read §2 defs:
  `graveyardOriginsAreDepartureObserved` (`Proofs/CardFidelity.lean:158`),
  `spelledGraveyardOriginsAreDepartureObserved` (`:172`), the `flawlessForgeryLine` pin
  (`Proofs/CardFidelity.lean:37`), `Proofs/Keyword.lean:458` and `:478`, `Proofs/Trigger.lean:352`,
  `Proofs/Counters.lean:781`, `costCoercionsKeepTheCard` (`Proofs/Authoring.lean:414`) and
  `Proofs/Mana.lean:385`. Where a pin states a term equality against a copied tree, update the
  copy in lockstep.
- Add one test: `seedbornMuseUntapsOnlyPermanents`, beside the carried-over
  `Proofs/CardFidelity.lean` tests, asserting that Seedborn Muse's untap patient carries
  `isPermanent` (over the spelled card's static).
- Retired: `okFiresongJoinEcho` (§3), because it asserts a non-card.

## Acceptance

- `cargo test` and `cargo clippy --tests -- -D warnings` for `semantics` and `discourse`.
- `rg 'allOf\(youControl\)' plugins_v3/canon/cards` is empty, and no carried-over fragment test
  writes `allOf youControl`; the 20 §1 rows and every §2 line carry the head noun.
- CR citation gates: `cargo xtask cite check --list-noncompliant` empty, `cargo xtask cite check`
  0 stale.

## Not in this ticket

- Rogue Class's and Summon: Esper Valigarmanda's `toCast (in_ (exiledWith …))`
  (`Proofs/CardsChoice.lean`) re-describe "those spells" without anaphora; that is
  `discourse-cast-permission-publishes-spell`, not a head-noun fix.
- Ice Cauldron (`Proofs/CardsMana.lean`) adds 1 mana for "this artifact's last noted type and
  amount of mana" and drops "last" from "the last card exiled with this artifact"; that is
  `lean-v3-cast-spell-anaphora-and-noted-mana` Part 2.

## Current whole-card paths

Fragment definitions keep their identifiers. Whole cards are looked up by printed name (Lean's
`cardNamed "<Printed Name>"` above). Edit the declarations at the following paths. Multipart keys
include both printed names.

| Printed name | Canon file |
|---|---|
| Abolish | `plugins_v3/canon/cards/Abolish.syl` |
| Ashes of the Fallen | `plugins_v3/canon/cards/Ashes of the Fallen.syl` |
| Aven Shrine | `plugins_v3/canon/cards/Aven Shrine.syl` |
| Blatant Thievery | `plugins_v3/canon/cards/Blatant Thievery.syl` |
| Blazing Shoal | `plugins_v3/canon/cards/Blazing Shoal.syl` |
| Bleeding Effect | `plugins_v3/canon/cards/Bleeding Effect.syl` |
| Bloodfire Enforcers | `plugins_v3/canon/cards/Bloodfire Enforcers.syl` |
| Cairn Wanderer | `plugins_v3/canon/cards/Cairn Wanderer.syl` |
| Carrion Grub | `plugins_v3/canon/cards/Carrion Grub.syl` |
| Celebrate the Harvest | `plugins_v3/canon/cards/Celebrate the Harvest.syl` |
| Charnel Troll | `plugins_v3/canon/cards/Charnel Troll.syl` |
| Chrome Mox | `plugins_v3/canon/cards/Chrome Mox.syl` |
| Cold Storage | `plugins_v3/canon/cards/Cold Storage.syl` |
| Crypt Lurker | `plugins_v3/canon/cards/Crypt Lurker.syl` |
| Cultivator Colossus | `plugins_v3/canon/cards/Cultivator Colossus.syl` |
| Custodi Squire | `plugins_v3/canon/cards/Custodi Squire.syl` |
| Death or Glory | `plugins_v3/canon/cards/Death or Glory.syl` |
| Death-Mask Duplicant | `plugins_v3/canon/cards/Death-Mask Duplicant.syl` |
| Delivery Moogle | `plugins_v3/canon/cards/Delivery Moogle.syl` |
| Disrupting Shoal | `plugins_v3/canon/cards/Disrupting Shoal.syl` |
| Drach'Nyen | `plugins_v3/canon/cards/Drach'Nyen.syl` |
| Force of Will | `plugins_v3/canon/cards/Force of Will.syl` |
| Garruk Relentless // Garruk, the Veil-Cursed | `plugins_v3/canon/cards/Garruk Relentless {slash}{slash} Garruk, the Veil-Cursed.syl` |
| Land Tax | `plugins_v3/canon/cards/Land Tax.syl` |
| Midnight Scavengers // Chittering Host | `plugins_v3/canon/cards/Midnight Scavengers {slash}{slash} Chittering Host.syl` |
| Mox Diamond | `plugins_v3/canon/cards/Mox Diamond.syl` |
| Nourishing Shoal | `plugins_v3/canon/cards/Nourishing Shoal.syl` |
| Old-Growth Dryads | `plugins_v3/canon/cards/Old-Growth Dryads.syl` |
| Rogues' Gallery | `plugins_v3/canon/cards/Rogues' Gallery.syl` |
| Rune Snag | `plugins_v3/canon/cards/Rune Snag.syl` |
| Scapeshift | `plugins_v3/canon/cards/Scapeshift.syl` |
| Seedborn Muse | `plugins_v3/canon/cards/Seedborn Muse.syl` |
| Shining Shoal | `plugins_v3/canon/cards/Shining Shoal.syl` |
| Sickening Shoal | `plugins_v3/canon/cards/Sickening Shoal.syl` |
| Sphinx of the Chimes | `plugins_v3/canon/cards/Sphinx of the Chimes.syl` |
| Sunscour | `plugins_v3/canon/cards/Sunscour.syl` |
| Syr Konrad, the Grim | `plugins_v3/canon/cards/Syr Konrad, the Grim.syl` |
| Thawing Glaciers | `plugins_v3/canon/cards/Thawing Glaciers.syl` |
| Tower Winder | `plugins_v3/canon/cards/Tower Winder.syl` |
| Urborg Scavengers | `plugins_v3/canon/cards/Urborg Scavengers.syl` |
| Voracious Brood | `plugins_v3/canon/cards/Voracious Brood.syl` |
| Vraska's Scorn | `plugins_v3/canon/cards/Vraska's Scorn.syl` |
