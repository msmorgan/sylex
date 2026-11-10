---
needs: []
---

# Give every description an explicit head noun

## Why

A description with a card type or subtype, no zone, and none of the words "card", "spell" or
"source" means a permanent of that type on the battlefield [CR#109.2]; "card" with a zone, "spell"
and "source" give it other domains [CR#109.2a,109.2b,109.2c]. "Target creature" is a creature
permanent, and "a creature card in your graveyard" is not one. Nothing in a term says so today:

- `isCreature` is `hasType(creature)`, and the other single-type Templates are likewise the bare
  type test. `isPermanent` (`in(battlefield)`) is written only where the text or a rule prints
  "permanent" or "on the battlefield".
- Every determiner Template (`target`, `a`, `each`, `bare`, `allOf`, `the`, `counted`, `countOf`,
  `exists`) is `described(<det>, $p)` with nothing added.
- Of about 1,075 type-predicate words in description positions under `plugins/` (the nine `isX`
  Templates, `hasType`, `hasSubtype`, `hasSupertype`), about 904 carry no domain word in their
  description; 61 carry `isSpell`, 40 a permanent or battlefield word, 36 a card word, 28 a zone
  without "card", 6 `isToken` alone. The 904 by consumer: `target` 191, `allOf` 175, `a` 161,
  `enchant` 80, `counted` 48, `exists` 42. These counts come from a study's own `.syl` parser and
  resolver, not the project's checker.
- `semantics.md` §2.4 reads [CR#109.2]'s carrier words for anaphors, and core keeps
  `Class(Permanent)` apart from the type atom (`crates/core/src/filter.rs`), but no design document
  says which stage turns "target creature" into a description of battlefield objects.

The same split is why `.syl` has no coercion from `CardType`, `Subtype` or `Supertype` into
`Predicate`. A printed "creature" means a creature on the battlefield, while in "creature card" the
word means only the type, so a bare `creature` at `Predicate` standing for `hasType(creature)`
would be subtly wrong wherever it heads a description. Choosing the meaning from the sibling
conjuncts (a description whose conjuncts include a type test and none of `isCard`, `cardIn`,
`isSpell`, `isSource` or `in(<zone>)` describes battlefield objects) makes a conjunction's meaning
depend on its siblings after expansion, and is hacky. So the type words stay wrapped (`isCreature`,
`hasSubtype(goblin)`) and the coercions stay three: `CardType` and `Subtype` into `NounWord`, an
integer into `Amount` (`syl.md` §2).

This ticket tracks the structural answer: a description whose head noun is a slot of its own, so a
type word's meaning is fixed by where it stands.

## Design

The idea, as a study worked it out. It is not adopted; the written forms below are warty (three
spellings, `pre`, `post` and `nominal`, for one phrase shape, two of them existing only so no `[]`
is written, and a head slot that turns every flat conjunction into nested calls).

- **One sort for a head with its modifiers**, today's `NounWord` widened (placeholder name
  `Nominal`). Every determiner Template, `described`, `enchant` and `affinity` take a `Nominal`
  where they take a `Predicate` today, as `that(creature)` and `enchanted(creature)` already do.
- **Three slots in printed order**: pre-head modifiers (`List(Predicate)`), the head (`Nominal`),
  post-head modifiers (`List(Predicate)`). "Legendary creature you control" is
  `target(nominal([legendary], creature, [youControl]))`. One-sided forms `pre(mods, head)` and
  `post(head, mods)` cover a nominal with modifiers on one side, so each printed nominal has one of
  four forms: the bare head, `pre`, `post`, `nominal`.
- **What a head word means.** A card type or subtype word: a permanent of that type
  [CR#109.2,110.1]. `permanent`: an object on the battlefield [CR#110.1]. `card`: a Magic card,
  never a token [CR#108.2,108.2b], in the zone a modifier names [CR#109.2a]. `spell`: a spell on the
  stack [CR#109.2b]. `source`: a source in any zone [CR#109.2c]. Also `token`, `player`,
  `opponent`, `outlaw` [CR#700.12a]. A supertype never heads a description.
- **A type word as a modifier or complement is a bare `Predicate`**, the type test and nothing
  more: "creature" in "creature card", `non(creature)` in "noncreature spell". With a head slot
  this reading is never a head, so `Predicate` coercions through `hasType`, `hasSubtype` and
  `hasSupertype` would no longer be subtly wrong. `permanent` likewise has two meanings the CR
  draws: as a head, on the battlefield [CR#110.1]; as a modifier ("permanent card"), a permanent
  type [CR#110.4a].
- **Coordination at the head.** `or`, `and` and `andOr` at `Nominal` coordinate heads and whole
  nominals; at `Predicate` they coordinate modifiers. *Haven of the Spirit Dragon* prints "target
  Dragon creature card or Ugin planeswalker card": one determiner over two whole nominals,
  `target(or([pre([dragon, creature], card), pre([ugin, planeswalker], card)]))`. *Ghostly Flame*
  prints "Black and/or red permanents and spells": one modifier over two heads. At `Nominal` every
  coordinator means union ("all artifacts and creatures"), which the flat form cannot write
  faithfully today (`and([isArtifact, isCreature])` is artifact creatures).
- **Two families where a type-word head is not a permanent now.** The object of `play`: *Crucible
  of Worlds* prints "You may play lands from your graveyard", and what is played is a card in a
  graveyard (the CR words it the same way, "To play a land, a player puts that land onto the
  battlefield from the zone it was in" [CR#116.2a]); `cast(a(spell))` has the same shape today.
  Departed objects named by what they were: *Greater Good*'s "the sacrificed creature's power",
  "each creature that died this turn". Both are read at another time than "now", which
  `semantics.md` §2.4 and §4.5 own; no lowering test yet shows `play(a(land))` works when `land`
  carries the permanent class.
- **Complements and qualities have no head.** *Prophecy* prints "Reveal the top card of target
  opponent's library. If it's a land, you gain 1 life": a type word in a complement about a card in
  a library. *Storm Elemental* prints "If the exiled card is a snow land". Protection from a card
  type, subtype or supertype applies to permanents and to sources not on the battlefield, which the
  rule states as an exception to the permanent reading [CR#702.16a,109.2]. These positions keep a
  bare `Predicate`.
- **`enchant` and affinity take headed descriptions with no determiner.** Enchant is "Enchant
  [object or player]" [CR#702.5a]: *Animate Dead*'s "Enchant creature card in a graveyard" is
  `enchant(nominal([creature], card, [in(graveyard)]))`. Affinity "for [text]" counts "each [text]
  you control" [CR#702.41a]: *Banish to Another Universe*'s "Affinity for historic permanents". A
  nominal is therefore a value of its own sort, not three extra parameters on each determiner.
- **Scale, by the study's own parser.** 1,510 description sites; 1,341 (89%) respell mechanically
  from today's term to a head and two lists, and about 97% of the checked canon ones name the
  printed head. Shapes: bare head 523, `post` 510, `pre` 169, both sides 166, coordination at the
  top 35. About 80 sites, almost all builtin Definition bodies, by hand, with about 35 signatures
  from `Predicate` to `Nominal`. 37% of sites get shorter, 12% (both sides) longer.
- **ADR text it contradicts**, each needing an amendment: `spelling.md` §2 (modifiers on both sides
  of a noun from one list are one-sided adjunction; this makes them substitution into two list
  Holes around a head Hole), §3.1 (the head stands among its modifiers in one printed-order list,
  and a modifier's side comes from its own Construction), §7 (a `Predicate` Hole may be a noun, as
  in "creature"; a `Predicate` would no longer stand for the noun a determiner belongs to), and §9,
  which lists "explicit premodifier and postmodifier slots in place of adjunction on a nominal"
  among rejected alternatives.
- **Alternatives weighed, and where each falls short.**
  - Template pairs for "X" and "X card": either the bare word is the head reading
    (`and([hasType(x), isPermanent])`) and some 210 modifier sites need generic wrappers per
    carrier (`cardOf`, `spellOf`, `non` retyped, each in a `CardType` and a `Subtype` variant, 12
    to 15 Templates), so `creature` stops meaning `hasType(creature)`; or the bare word is
    `hasType` and the head is wrapped, which is one of the next two.
  - `isPermanent` written wherever [CR#109.2] applies: about 900 sites gain a word the card does
    not print, or a Template that prints nothing.
  - Inference in `described`: no respell and nothing stored, but a conjunction's meaning depends on
    its siblings after expansion; it is right only where "card" and "spell" are written (42 canon
    sites drop them); a Definition's parameter changes the meaning of the conjunction it lands in
    (affinity's `and([$quality, youControl])`); protection, complements and `non` each need a
    stated exemption.
  - Head first with one modifier list, `n(creature, [legendary, youControl])`: one list and sides
    from Constructions, but the head is read before a premodifier printed before it, an exception
    to `syl.md` §5, and a premodifier can hold a reference ("another" reads its Contrast Anchor,
    "exiled" a deed stamp).
  - A head marker in the flat list (`head(creature)`) makes "exactly one head" a checker rule and
    writes a marker at every site; a head per determiner (`targetCard`, `aSpell`) multiplies
    Templates and leaves `enchant` without one.
- **The study's weak points.** Its counts come from its own reimplementation of resolution, not
  the checker. Its head-picking rule and its table of which side each modifier prints on are its
  own; `was(…)` prints on either side ("the exiled card", "card exiled this way"), and 31 sites
  are not in pre, head, post order today. Its split of Oracle mismatches into noise and real
  defects was done by eye. Un-sets were not probed. The `play` and departed-object families rest
  on an untested reading in lowering. The head form cannot write a head printed before its
  premodifier, which is `syl-list-element-printed-order`'s main case.

## Open questions

- **D1. An explicit head at all**, amending `spelling.md` §2, §3.1, §7 and §9, or flat predicates
  with one of the alternatives above.
- **D2. The written form of a modified nominal**: (a) `nominal(pre, head, post)` with one-sided
  `pre` and `post`; (b) head first with one modifier list; (c) the three slots on every determiner
  Template, which leaves `enchant` and coordination without a nominal value. Whichever, it must be
  less warty than (a).
- **D3. A predicate complement** ("is a land", "is a creature card"): (a) a bare `Predicate`, no
  head, no [CR#109.2]; (b) a `Nominal`, which misreads *Prophecy* and *Storm Elemental*.
- **D4. What head `token` means**: (a) a token on the battlefield, since a token elsewhere ceases
  to exist [CR#111.7]; (b) the token test alone.
- **D5. Where [CR#109.2] is written**: (a) in the head primitives `type` and `subtype`, with
  lowering emitting the permanent class, keeping the coercion body rule ("a primitive at its sort
  applied to that parameter alone"); (b) relax the coercion body rule.
- **D6. The sort's name**: `Nominal` (`english.md` already uses the term) or `NounWord`.
- **D7. Order of work**: restore the printed heads first (`syl-card-head-noun-sweep`), then one
  scripted respell gated on expanded-tree equality except at head positions; whether `Predicate`
  coercions land in the same respell.

## Done when

- D1 is answered. If no, this ticket is deleted. If yes, D2 to D7 are answered, the ADR
  amendments are written, and the ticket is rewritten as planned work with checkable criteria.

## Not in scope

- Defects in today's description terms that are wrong under every option (dropped "card" or
  "permanent", unprinted type words, "on the battlefield" written as `isPermanent`):
  `syl-card-head-noun-sweep`.

## Pointers

- `docs/decisions/syl.md` §2, §3, §5; `docs/decisions/spelling.md` §2, §3.1, §7, §9;
  `docs/decisions/semantics.md` §2.4, §4.5
- `syl-card-head-noun-sweep`, `syl-opponent-noun-and-predicate`, `syl-list-element-printed-order`
- `plugins/builtin/coercions/`, `plugins/builtin/templates/predicate/`,
  `plugins/builtin/primitives/noun/`, `crates/xtask/src/dsl_check/coercion.rs`,
  `crates/core/src/filter.rs`
