# Kind sorts, membership and two modes: a describing phrase is declared once, at its kind of phrase

A phrase that describes something is declared at the sort that names its kind: an adjective
phrase, a preposition phrase, a relative clause, a participle, a nominal or a noun phrase. The
places such a phrase stands (before a head, after it, after a copula) are sorts whose members are
those kinds, so one declaration serves every place its kind is admitted. Membership replaces
coercion. What a describing phrase means is an unspelled test at a sort of its own, which a
card's term cannot reach and a Template's body can. A coordinator is declared once for the sorts
it coordinates and stands at the lowest sort that holds its arms.

The `.syl` below is the shape of the design. It has not been run through `cargo xtask dsl check`.
Counts are Corpus Units of Vintage-legal cards in `data/derived/english-forests.sqlite` in which
some Derivation has the phrase in that function; they are upper bounds. Site counts are the call
sites the checker resolves to a declaration, builtin / canon.

## 1. The kind sorts

Each is named for the English grammar's Category of the same name
(`crates/english/grammar/english.fug`).

| Sort | A phrase of it | Stands |
|---|---|---|
| `AdjectivePhrase` (NEW) | "red", "legendary", "tapped", "other", "other than X", "tied for …" | bare: before a head and after a copula ("tapped": 353 before, 230 after); with a complement: after a head and after a copula ("other than": 167 after a head) |
| `PrepositionPhrase` (NEW) | "in your graveyard", "with flying", "of the chosen type", "face down" | after a head ("in": 1,624); after a copula for some ("in": 313) |
| `RelativeClause` (NEW) | "you control", "that's white or blue", "that could enchant it" | after a head only ("control" as a bare relative: 9,044; *Glorious Anthem*) |
| `Participle` (NEW) | "attacking", "blocked", "named X", "attached to it", "controlled by X" | a verb phrase in a participle form: bare before a head (*Smite*'s "target blocked creature"; "attacking" 604, "chosen" 411, "exiled" 228), with dependents after it; under "be" it is the passive or the progressive |
| `Nominal` | "creature", "Goblin", "token", "permanent" | before a head ("creature": 5,047), and as a head |
| `NounPhrase` | "the monarch", "a Kree" | after a copula |

A phrase is declared once, at its kind. "Become" and "remain" take the complement "be" takes
("tapped": 65 under be, 123 under become, 42 under remain), and so does "turn" ("turn target
creature face down", about 180 units).

## 2. Membership

**A sort may have member sorts.** Every value of a member is a value of the sort, written as
itself: membership has no body and builds nothing. It is transitive, and it replaces coercion.

A membership is a declaration with no name and no body, in `plugins/builtin/members/`. Its
subject is a sort, which `sylex_syl`'s parser learns to read in that place:

    member(Color): AdjectivePhrase
    member(AdjectivePhrase): PreModifier

| Sort | Members |
|---|---|
| `PreModifier` | `AdjectivePhrase`, `Nominal`, `Participle` |
| `PostModifier` | `PrepositionPhrase`, `RelativeClause`, `AdjectivePhrase`, `Participle` |
| `Predicative` | `AdjectivePhrase`, `PrepositionPhrase`, `NounPhrase` |
| `AdjectivePhrase` | `Color`, `Supertype` |
| `Nominal` | `CardType`, `Subtype` |
| `Amount` | `Int` |

Under each sort its members form a tree: no sort reaches another by two paths, and load refuses
a cycle or a second path.

- `PreModifier` and `PostModifier` type the `pre` and `post` lists of `nominal`, and `Predicative`
  the complement of `is` and `thatIs`, as today. They are the **position sorts**: each names a
  place, and its members are the kinds that stand there.
- `post` holds every dependent printed after the head, in printed order: a complement the head
  selects ("half of this split card") and a modifier ("you control") alike. The term does not mark
  which a phrase is.
- **Membership is by kind.** A position sort admits every phrase of a member kind, including
  wordings English does not print there: "is other", "other than X creature", "legendary" after a
  head. No finer sort refuses them; semantics rejects what means nothing.

**A verb phrase is never a member of a position sort; a participle is.** A participle is a form
of a verb phrase, not a subset of the verb phrases: "copy target spell" cannot modify a head, and
"copied" can. The pool prints a gerund-participle before a head for three verbs only (`attack`
604 units, `block` 150, `defend` 7) and a past participle for many ("chosen" 411, "exiled" 228,
"dealt" 196). So `Participle` is a kind sort of its own, and the operators that make one from a
verb phrase produce it: `done(phrase, by)`, the passive participle
(`docs/specs/voice-and-object-phrases.md` §3), and `ing(phrase)` (NEW), the active one, spelled
"$phrase@GerundParticiple". "The copied creature" is `done(copy(subject))` in `pre`, and "a
creature attacking you" `ing(attack(defender = you))` in `post`. `be` takes a `Participle`. A
noun and a verb of one name (`copy` today; `counter`, `die`, `flip`, `roll`, `vote` and `target`
as their nouns are declared) therefore never meet in one position. Until
`syl-passive-participle-and-trigger-events` declares `done` and `ing`, each participle the bundle
needs is a fixed Template or primitive at `Participle` (`attacking`, `blocked`, `named`,
`attachedTo`, `controlledBy`).

What a position sort does not hold: the passive and the progressive ("is attached to it", "isn't
being declared as an attacker", "that's attacking you") are `be` over a participle, and a "has …"
clause is a verb phrase headed by "have".

**What the nine coercions become.** Each wrapped a word through a primitive (`hasColor($color)`,
`type($type)`, `lit($value)`); the word is now a member and the wrapper goes, so the bare word is
the only written form.

- `CardType` and `Subtype` are members of `Nominal`, and `Nominal` of `PreModifier`, so a type or
  subtype word is one word wherever it stands: "creature" in "creature card" is the word the head
  "creature" is. `token`, `permanent` and `outlaw` are each one declaration as head and as
  modifier, where today the `PreModifier` copy builds a different node (`token` a token test,
  `permanent` "has a permanent card type") from the head's. Which is meant is read from where the
  word stands, as is the rule that a description with a type word, no zone and no "card", "spell"
  or "source" means a permanent on the battlefield [CR#109.2]; the term does not write it. The
  `Nominal` primitives `type` and `subtype` go.
- `Color` and `Supertype` are members of `AdjectivePhrase`, so a supertype word stands after a
  copula as a color does.
- `Int` is a member of `Amount`, and `lit` goes.
- `Status` is a member of nothing. It stays the sort of the game's statuses, which `setStatus`,
  `becomeStatus`, `entersAs` and the test `hasStatus` take, and its words carry no spelling. Each
  wording is a Template at its kind over `hasStatus`: `tapped` and `untapped` at
  `AdjectivePhrase`; `faceDown`, `faceUp`, `phasedIn` and `phasedOut` at `PrepositionPhrase`,
  spelled "face down", "face up", "phased in" and "phased out". The hyphenated "face-down" (60
  units), "face-up" (17) and "phased-out" (1) stand only before a head and are `PreModifier`
  Templates, `faceDown`, `faceUp` and `phasedOut`; `PrepositionPhrase` is no member of
  `PreModifier`, so the two `faceDown`s share no position. When `syl-become-replaces-types` gives
  "become" a `Predicative` complement, `becomeStatus` goes, so "becomes tapped" keeps one written
  form.
- The words of the wrappers' names at `Predicate` (`hasType`, `hasColor` and the rest) and
  `subtype` at the sorts a text change and a choice read are not wrappers and stay.

## 3. Tests, and the two modes

**`Test` (NEW) holds what a describing phrase means.** About thirty primitives say what a phrase
tests and print nothing: `hasPossessor`, `inCombat`, `compare`, `shares`, `superlative`,
`colorCount`, `currentFace`, `hasDesignation`. Today each is copied per position sort, the
Templates that carry the wordings are written over them ("you control" is `hasPossessor(…)`,
"with power 3 or less" is `compare(…)`, "attacking" is `inCombat(…)`), and rule bodies and 44
card files also call them bare. A test is no kind of phrase. It is declared once, at `Test`, and
`Test` is a member of nothing.

**A term is resolved in one of two modes.**

- **Surface mode** resolves a `card`'s term. At expected sort `T` a value stands at `T` or at a
  sort that reaches `T` through membership, and nowhere else. No surface sort reaches `Test`, so
  a card cannot write a test: *Propaganda*'s `inCombat(attackerOf, counterpart = you)` in `post`
  is refused by its sort, where today nothing refuses it and nothing prints it.
- **Body mode** resolves the body of a Template, a Definition or a rule, and a parameter's
  default. It adds one relation, declared per kind:

      denotes(AdjectivePhrase): Test
      denotes(PrepositionPhrase): Test
      denotes(RelativeClause): Test
      denotes(Participle): Test

  A kind that denotes `Test` expands to a test. In body mode a slot typed by such a kind, or by a
  position sort, also admits a `Test`, and a slot typed `Test` also admits a phrase of such a
  kind. So `colorless`, an `AdjectivePhrase`, has the body `colorCount(equal, 0)`;
  `youDontControl` is `not(controls(you))`, a test's `not` over a relative clause; and a rule
  body writes `post = [hasPossessor(controller, $player)]` as it does now. `Nominal`,
  `NounPhrase` and `VerbPhrase` denote themselves: a type word before a head is read where it
  stands (§2), not expanded to a test.

A declaration's signature and `spelling` are surface; its body is in body mode. After expansion
every describing phrase is a `Test`, so expansion keeps each slot well sorted, and a parameter
may be typed by a kind.

**Reaches.** SurfaceReaches(`T`) is `T` and every sort that reaches it through `member`.
BodyReaches(`Test`) is `Test` and SurfaceReaches of each kind that denotes it; BodyReaches of
such a kind, or of a position sort, is its SurfaceReaches and `Test`; BodyReaches of any other
sort is its SurfaceReaches. A word declared at two sorts of one Reaches set, in either mode, is
refused at load, as `syl.md` §3 refuses it for coercions today, and so is a literal that stands
at two (an integer when `Int` and `Nat` both reach a sort). Over §5's placement no word is
(`docs/evidence/syl-kind-sorts/reframe-check.md`).

**Resolution.** `syl.md` §3's order keeps four steps, and its fourth changes. At expected sort
`T`, a value resolves:

1. `none`, if `T` is optional;
2. what stands at `T` itself;
3. if `T` is `Option(U)`, the value at `U`, by steps 2 and 4;
4. at the one sort in the mode's Reaches(`T`) that the value stands at.

**A position sort's own declarations** are Templates for wordings bound to that position:
"face-down" before a head; `withMostVotes`, "with the most votes or tied for most votes", a
coordination of unlike kinds fixed in one wording. Anything else at a position sort is refused at
load, except the declarations §5 lists as staying, which the check names with their tickets.

**Nothing at `Test` prints.** No surface sort reaches it, so a `spelling` there would never be
used; load refuses one, apart from the words of §4, which stand at `Test` as one of several
sorts.

**Calls of a test in a card.** Forty-nine calls in 44 canon files write a test bare where the
card prints a phrase no declaration spells. Surface mode refuses each. Four take a spelled phrase
that this design gives them (§5); the rest are named in the check as exceptions, resolved in body
mode, until `syl-spelled-test-phrases` writes their phrases.

## 4. Coordination and negation

**A coordinator is declared once, for the sorts it coordinates.** Today `or`, `and`, `andOr`,
`both` and `coord` are separate declarations at each of `Nominal`, `NounPhrase`, `PreModifier`,
`PostModifier`, `Predicative` and nine other sorts. One declaration lists its sorts, with `Self`
for the sort it stands at:

    template or(items: List(Self)): Self {
      at = [AdjectivePhrase, PrepositionPhrase, RelativeClause, Participle, Nominal, NounPhrase,
            PreModifier, PostModifier, Predicative, Test],
      spelling = "$items#<or>",
      body = coord(or, $items)
    }

`and`, `andOr`, `both` and the primitive `coord` are declared the same way, each with the sorts
the pool coordinates with it. The coordinators at `Cost`, `Condition`, `Timing`, `Location`,
`GameEvent`, `TriggerCondition`, `ProducedMana` and `Predicate` stay separate declarations;
`syl-coordinators-outside-descriptions` (NEW) folds them in.

**A call of such a word stands at the lowest of its sorts that holds every arm.** Each arm has
one sort: its head word's, which is unique in Reaches(`T`); a parameter's declared sort; or, for
a nested coordination, the sort it resolved at. Reaches(`T`) is a tree, so the arms' sorts have
one lowest common sort, and the call resolves at the lowest sort in the word's `at` list at or
above it. In body mode `Test` is tried last, when no surface sort holds the arms.

- `or([white, blue])` in `pre` or after "that's": both arms are colors, so it stands at
  `AdjectivePhrase`. The grammar parses "white or blue" as a coordination of adjective phrases in
  both places (*Fry*).
- `or([knight, aura, equipment, nominal(pre = [legendary], artifact)])` in `pre`: the arms are
  subtypes and a nominal, so it stands at `Nominal`, which is how the grammar parses *Acclaimed
  Contender*'s "Knight, Aura, Equipment, or legendary artifact card" in the Reading that attaches
  "legendary" to "artifact". `stacked` goes.
- A coordination of an adjective and a participle ("tapped and attacking") stands at the position
  sort, where the grammar puts a coordination of unlike kinds.
- `or([bare(artifact), bare(creature), bare(enchantment), bare(land)])` is *Solemnity*'s
  "artifacts, creatures, enchantments, or lands", a coordination of noun phrases. The grammar has
  no bare plural over coordinated nominals, so `bare(or([…]))`, which *Solemnity*, *Clarion
  Conqueror*, *Damping Matrix* and *Grand Abolisher* write today, is no Reading's term.

This is the one place resolution reads a call's arguments. It stays one pass with no
backtracking, because an arm's sort never depends on the coordinator's. Of the 165 coordinator
calls in `plugins/` at these sorts, 121 stand where they do today, 29 move from `PreModifier` to
`Nominal`, 3 from `Predicative` to `AdjectivePhrase`, 2 from `PostModifier` to
`PrepositionPhrase`, 2 from `PreModifier` to `Participle`, and 2 need the rewrites §5 gives them
(`historic`, *Acclaimed Contender*); each of the 13 sampled calls that lands at a kind has its
coordination at that Category in the card's parse.

**A nominal in `pre`** is a bare head word (`creature`, `goblin`, `token`) or a coordination.
`nominal(…)` with a `pre` or `post` of its own is refused there except as an arm of a
coordination, since `nominal(pre = [nominal(pre = [legendary], creature)], card)` and
`nominal(pre = [legendary, creature], card)` would be two written forms of "legendary creature
card".

**Negation is three constructions.**

- **The prefix "non-"** attaches to a nominal ("nonland", 1,813 units), an adjective ("nonbasic",
  245) or a participle ("nonattacking", 5), and the result is of the same kind: `non` is declared
  once `at = [AdjectivePhrase, Nominal, Participle]`. So "If that land was nonbasic" (*Molten
  Rain*) and "nonlands" as a head have terms, which `non` at `PreModifier` did not give them.
- **"Not" before a phrase** stands before an adjective phrase ("that's not historic",
  *Desynchronization*), a preposition phrase ("card not on the battlefield", *Anafenza, the
  Foremost*), a noun phrase ("if it's not a Kree", *Captain Marvel, Apex Avenger*) or a participle
  ("not named …", *Escaped Shapeshifter*): `not` is declared once `at = [AdjectivePhrase,
  PrepositionPhrase, NounPhrase, Participle, Test]`. It never stands before a head ("Exile target
  not green creature." has no Reading), and `not` at `PreModifier` goes.
- **"Isn't" and "aren't"** negate the copula, not its complement: "Creatures that aren't of the
  chosen type" (*Crippling Fear*) parses with the verb "aren't" over "of the chosen type". They
  are `isnt` and `thatIsnt` (NEW) beside `is` and `thatIs`.

## 5. What each declaration becomes

Every declaration at `PreModifier`, `PostModifier` and `Predicative` in `plugins/builtin/`: 135.
The placement follows the checker's own resolutions (`docs/evidence/syl-kind-sorts/`:
`test-sort-check.md` for the placement, `reframe-check.md` for the two modes and the
coordinators). Where a row names no card, the declaration is called only from rule bodies and no
Reading was checked.

### Moves to its kind

| Kind | Declarations | Decided by |
|---|---|---|
| `AdjectivePhrase` | `other`, `otherThan`, `split`, `colorless`, `multicolored`, `historic`, `unblocked`, `transformed`, `tiedFor`, `plotted`; the new `tapped` and `untapped`; the color and supertype words by membership | "red" 496 before a head, 21 after a copula (*Fry*); "legendary" 334 / 74; "colorless" 539 / 15; "multicolored" 64 / 1; "historic" 60 / 2 (*Curator's Ward*'s "if it was historic"); "unblocked" 13 / 4; "other" 1,567 before a head; "other than" 167 after a head; "tied" 34 after a copula; "tapped" 353 / 230; "plotted" only after "becomes" (7 units, *Aven Interrupter*) |
| `PrepositionPhrase` | `in`, `inPile`, `of`, `fromSource`, `ofChosen`, `ofYourChoice`, `ofTheLastChosen`, `hasPower`, `hasManaValue`, `hasLesserManaValueThan`, `activationCostHas`, `manaCostHas`, `withDifferentNames`, `withTheSameName`, `outsideTheGame`; `withCounter` and `withKeyword` (below); the new `faceDown`, `faceUp`, `phasedIn`, `phasedOut` | "in" 1,624 after a head, 313 after a copula; *Aerial Volley* "with flying"; *Abrupt Decay* "with mana value 3 or less"; *Cursed Totem* "of creatures"; *Gaea's Revenge* "from nongreen sources"; *Adaptive Automaton* and *Crippling Fear* "of the chosen type"; *Unable to Scream* "is face down" |
| `RelativeClause` | `youControl`, `youDontControl`, `youOwn`, `opponentsControl`, `anOpponentControls`, `controls` (below), `ownedBy`, `actorControls`; `thatIs`, `couldEnchant`, `triggeredNotLeftStack`; `did` | *Glorious Anthem* "you control"; *Academy Journeymage* "an opponent controls"; *Fry* "that's white or blue"; *Bruna, Light of Alabaster* "that could enchant it" |
| `Participle` | `attacking`, `blocking`, `blocked`, `chosen`, `equipped`, `foretold`, `attachedTo`, `named`, `notChosen`; `was`, one Template for its `PreModifier` and `PostModifier` copies; the new `controlledBy` ("controlled by $who") | *Smite* "blocked", *Stone Haven Outfitter* "Equipped", *Bioplasm* "exiled"; "attach" passive 178 after a head; "name" passive 333; *Burn at the Stake* "tapped this way"; "controlled" 32 after a head (*Balance*) |

- **Three renames, each one name for two wordings today.** `controlledBy` at `PostModifier`
  spells "$who control" and at `Predicative` "controlled by $who": the relative is `controls`,
  and `controlledBy` names the participle. `hasCounter` and `hasKeyword` at `PostModifier` spell
  "with a counter on it" and "with $keyword" and at `Predicative` are the unspelled tests: the
  with-phrases are `withCounter` and `withKeyword`, Templates over the tests of the old names.
- **Bodies rewritten.** `historic` is `or([hasType(artifact), legendary, hasSubtype(saga)])`, a
  test's `or` over two tests and an adjective; `was` is written over `predicateDid`, the test
  `did` is over. `youDontControl` and `outsideTheGame` keep the bodies they have, which body mode
  types.
- **Cards that take the spelled phrase.** *Blessed Reversal*, *Tidal Flats*, *Beckoning
  Will-o'-Wisp* and *Triarch Stalker* print "attacking X" or "blocking X" after a head and write
  `attacking(…)` and `blocking(…)`, each with an optional counterpart, where they call `inCombat`
  today. *Propaganda*, *Ghostly Prison* and *Awesome Presence* print a relative, "that's
  attacking you", and are among the exceptions of §3.

### Declared once at `Test`

`agree`, `attachment`, `choseExtreme`, `chosenRef`, `colorCount`, `compare`, `currentFace`,
`hasCounter`, `hasDesignation`, `hasKeyword`, `hasPossessor`, `hasStatus`, `inCombat`,
`legalAttachment` (as `isLegalAttachment`, since a `Nominal` has the name), `predicateDid`,
`related`, `shares`, `superlative`, `targeting`, with the new unspelled `hasType` and
`hasSubtype`, and `tapOrUntapInCost`, which transcribes [CR#302.6]'s "the tap symbol or the
untap symbol", printed by no card, and loses its spelling. Each replaces its copies at the
position sorts. Every copy of a test builds one constructor, and where signatures differ the
longer one has the shorter's parameters with the rest optional (`inCombat`, `hasDesignation`,
and `attachment` with `word` passed by name), so every call resolves unchanged.

### Deleted

- The nine coercions, and the wrappers `hasColor`, `hasSupertype`, `hasType` and `hasSubtype` at
  `PreModifier`, `hasColor` and `hasStatus` at `Predicative`, `type`, `subtype` and `lit`.
- The copies of each test at a second and third position sort (fourteen).
- `in`, `ofChosen`, `otherThan`, `outsideTheGame` and `transformed` at `Predicative`: the
  declaration that moves, now reached by membership.
- `token`, `permanent` and `outlaw` at `PreModifier`: the `Nominal` word (§2). `tapped` and
  `untapped` at `PreModifier`: the adjective phrase.
- `noun` at `Predicative`: *Archon of Coronation*'s "you're the monarch" is the noun phrase
  itself.
- The coordinators and negations of the three position sorts (`or`, `andOr`, `and`, `both`,
  `coord`, `non`, `not`), those of `Nominal` and `NounPhrase`, `stacked`, and the `PreModifier`
  copy of `was`: §4's declarations stand in their place. Each caller of `not` at `PostModifier`
  writes what its card prints: `not` before a preposition phrase or a participle, `thatIsnt`, or
  a `without` preposition phrase ("without flying").

### The position sort's own

- `PreModifier`: `faceDown` ("face-down"), `faceUp` ("face-up", NEW) and `phasedOut`
  ("phased-out").
- `PostModifier`: `withMostVotes` (*Council's Judgment*).
- `Predicative`: `hasTypeIgnoringTypeChanges` (sunburst's rule).

### Stays until another ticket composes it

| Declarations | Prints | Ticket |
|---|---|---|
| `attachedTo`, `attachedBy`, `controlledBy`, `named`, `was` at `Predicative` | "are already attached to it" (*Benevolent Blessing*), "was cast from your hand" (*Approach of the Second Sun*): the passive, `be(done(…))` | `syl-passive-participle-and-trigger-events` |
| `did` at `Predicative` | "if you cast it" (*Arcane Proxy*): `did(phrase, within)` under `holds` | `syl-lookback-operator-and-holds` |
| `hasQuality` at `PreModifier` | 23 rule bodies put a keyword's quality in `pre` through it | `syl-keyword-quality-union` (NEW) |
| `copied` at `PreModifier` | its callers print "the copy" (*Rings of Brighthearth*) | `syl-canon-wording-defects` |

### `Predicate`

This spec leaves `Predicate` and its 36 declarations as they are. Thirteen keywords take their
quality at it (`protection`, `landwalk`, `affinity`, `enchant` and nine more), and about twenty
other parameters take one (`behold`, `transform`'s `into`, `resulting` on `cast`, `except` on
`allExcept`, `holders` on `keywordGrant`, `matching`, `oneEachOf` and others).
`syl-keyword-quality-union` decides what a keyword's quality is typed by and moves what is left
of `Predicate`'s tests to `Test`.

## 6. What a card's term may hold

Surface mode keeps a test out of a card. Two further rules would make "every word a card's term
may hold has a `spelling`" (`spelling.md` §7) something the checker enforces, and neither holds
today:

- **No unspelled declaration in a card's text.** All 981 canon card files call one:
  `singleFaced` (954 files), `static` (420), `actor` (409), `sequentially` (205), `ofChosen`
  (38), `costShift` (36), `preventDamage` (29), `visibility` (20), `establish` (17). Some are
  structure that prints nothing of its own (`singleFaced`; `static`, whose wording is its
  clause's); the rest are wordings not yet spelled.
- **Every Hole of a `spelling` has a spelled sort.** Thirty-seven Holes are typed by a sort with
  no spelled declaration. `hasPower` spells "with power $cmp $amt" over `Comparator`, which has
  none; cards print "with power 2", "with power 4 or greater" (*Elspeth, Sun's Champion*) and
  "with power greater than …" (*Ensnaring Bridge*), three wordings.

`syl-surface-admissibility` (NEW) builds both.

## 7. Kinds the pool prints that this spec does not place

| Position | Phrase | Units |
|---|---|---|
| before a head | a keyword ("Equip abilities", "flashback cost") | 163 |
| before a head | a power/toughness measure outside tokens and counters ("Target 1/1 creature", *Aegis of the Meek*) | 12 |
| before a head | a name ("Wicked Role token") | 69 |
| before a head | mana symbols ("{2}{U} kicker") | 30 |
| after a head | a temporal noun phrase ("the first time each turn") | 131 |
| after a copula | a measure ("except the token is 1/1", *Endless Evil*) | 23 |

A counter's kind before "counter" is no member of `PreModifier`: it is the argument of the head
`counter` (`syl-counters-noun-phrase`). The rest are `syl-unplaced-describing-kinds` (NEW). A
second open catalog of words is given a place before a head through a word that wraps it, not
through membership beside `Subtype`, whose 514 words it would collide with.

## 8. Tickets

Built by:

1. **`syl-union-membership`** (NEW): `member`, `denotes` and the `at` list in `sylex_syl` and
   `cargo xtask dsl check`, with coercion removed; the two modes; §4's coordinators and negation;
   the sorts of §1 to §3; §5's moves, tests, deletions and renames. It needs none of the verb
   tickets.
2. **`syl-spelled-test-phrases`** (NEW): the wordings cards print where they call a test bare,
   each leaving §3's exceptions.
3. **`syl-surface-admissibility`** (NEW), **`syl-keyword-quality-union`** (NEW),
   **`syl-have-verb-phrases`** (NEW), **`syl-unplaced-describing-kinds`** (NEW) and
   **`syl-coordinators-outside-descriptions`** (NEW).

Changed by this spec: `docs/specs/voice-and-object-phrases.md` and
`syl-passive-participle-and-trigger-events` (`done` and `ing` produce a `Participle`, `be` takes
one, and a verb phrase is no member of a position sort), `syl-lookback-operator-and-holds` and
`syl-clause-and-imperative` (`did` and `was`; a relative composed from a clause with a gap, at
`RelativeClause`), `syl-zone-words` (a location phrase is a `PrepositionPhrase`),
`syl-damage-and-life-noun-phrases` (its modifiers are declared at kinds),
`syl-become-replaces-types` (it consumes `Predicative`, and `becomeStatus` goes with it),
`syl-token-records-to-templates`, `syl-canon-wording-defects` (`copied`, `choseExtreme`),
`syl-continuous-effect-subject` (a membership where it plans a coercion), and
`syl-name-originally-printed-in`, `syl-daybound-reconfigure-bodies` and
`syl-clone-any-on-the-battlefield` (each asks for a new word at `Predicate`, which is declared at
its kind instead). `syl-verb-phrase-dependents` gives a verb phrase a dependent list typed by one
more position sort over the same kinds ("enters tapped", "exile it face down", "tapped and
attacking"), with no test in it and no coordinator of its own to declare; `MoveRider` stays until
it does.

## 9. Not decided here

- A typed Rust representation of the surface term, in which a sort with members is an enum over
  them (`PreModifier::Adjective(Adjective::Color(Color::Red))`) and the expanded tree has one
  `Test` type for every describing phrase.
- "Neither … nor", "but not" and the other correlative and contrastive coordinations.

## 10. What the design docs say once this is built

| Document | Today | Then |
|---|---|---|
| `syl.md` §2 | "No conversions besides `Option` and the declared coercions"; "There are nine" | no conversions; the `member` and `denotes` declarations, §2's memberships, and a declaration's `at` list |
| `syl.md` §3 | step 4 is a coercion, one hop; Reaches(`T`) is `T` and the from-sorts of coercions into it; a call resolves from its head word and the expected sort alone | §3 here; §4's rule for a word with an `at` list |
| `syl.md` §4 | a Template's body is a term of the Template's sort | resolved in body mode (§3) |
| `surface.md` §1 | "`Nominal` and its modifier sorts"; "the sorts of the words a coercion wraps (`CardType` under `Nominal`)" | the kind sorts of §1; a position sort names a place kinds stand; a word sort is a member of its kind |
| `surface.md` §4 | "Templates and Definitions are surface to surface. A body is a surface term" | a body is in body mode and may hold a test |
| `spelling.md` §2 | `stacked` for an arm of two premodifiers; `thatIs(predicative)` at `PostModifier`; a color at `Predicative` "through its coercion" | a nominal arm (§4); `thatIs` at `RelativeClause`; membership |
| `spelling.md` §4 | the bare word a coercion wraps and the written call are two written forms, and recovery yields the bare one | one written form |
| `spelling.md` §7 | "A sort is not one Category: a `PreModifier` is a noun in 'creature card' and an adjective in 'tapped creature'" | a kind sort is one Category and a position sort a function; the skeleton rule over Reaches |
| `semantics.md` §2.4 | "at `PreModifier` it tests the characteristic alone"; "`Predicative`'s noun-phrase form `noun(phrase)`" | the type word's reading is positional (§2); a noun phrase is a member of `Predicative` |

Glossary entries (`docs/contexts/game-model/CONTEXT.md`):

- **Coercion**: deleted. **Membership** (NEW): a sort's values are also values of each sort it is
  a member of.
- **PreModifier**, **PostModifier**, **Predicative**: each is a position sort, defined by its
  members and the place it types.
- **Test** (NEW): an unspelled primitive that says what a describing phrase means. **Surface
  mode** and **Body mode** (NEW). **Predicate** keeps its entry until its last consumer leaves.
