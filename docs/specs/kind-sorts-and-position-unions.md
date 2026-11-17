# Kind sorts and membership: a describing phrase is declared once, at its kind of phrase

A phrase that describes something is declared at the sort that names its kind: an adjective
phrase, a preposition phrase, a relative clause, a verb phrase, a nominal or a noun phrase. The
places such a phrase stands (before a head, after it, after a copula) are sorts whose members are
those kinds, so one declaration serves every place its kind is admitted. Membership replaces
coercion: a color word is a member of the adjective phrases as an adjective phrase is a member of
the premodifiers, and nothing wraps either. What such a phrase means is an unspelled test,
declared once at a sort of its own.

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
| `VerbPhrase` | as a participle: "attacking", "blocked", "named X", "attached to it" | bare before a head (*Smite*'s "target blocked creature"; "attacking" 485, "chosen" 411, "exiled" 228), with dependents after it; under "be" it is the passive or the progressive, not a predicative complement |
| `Nominal` | "creature", "Goblin", "token", "permanent" | before a head ("creature": 5,047), and as a head |
| `NounPhrase` | "the monarch", "a Kree" | after a copula |

A phrase is declared once, at its kind. "Become" and "remain" take the complement "be" takes
("tapped": 65 under be, 123 under become, 42 under remain), and so does "turn" ("turn target
creature face down", about 180 units).

## 2. Membership, and the unspelled tests

**A sort may have member sorts.** Every value of a member is a value of the sort, written as
itself: membership has no body and builds nothing. It is transitive, and it replaces coercion.

A membership is a declaration with no name, in `plugins/builtin/members/`, written as a coercion
is today with no body:

    member(Color): AdjectivePhrase
    member(AdjectivePhrase): PreModifier

| Sort | Members |
|---|---|
| `PreModifier` | `AdjectivePhrase`, `Nominal`, `Participle`, `Test` |
| `PostModifier` | `PrepositionPhrase`, `RelativeClause`, `AdjectivePhrase`, `Participle`, `Test` |
| `Predicative` | `AdjectivePhrase`, `PrepositionPhrase`, `NounPhrase`, `Test` |
| `AdjectivePhrase` | `Color`, `Supertype` |
| `Nominal` | `CardType`, `Subtype` |
| `Amount` | `Int` |

No sort reaches another by two paths.

- `PreModifier` and `PostModifier` type the `pre` and `post` lists of `nominal`, and `Predicative`
  the complement of `is` and `thatIs`, as today. They are the **position sorts**: each names a
  place, and its members are the kinds that stand there.
- `post` holds every dependent printed after the head, in printed order: a complement the head
  selects ("half of this split card") and a modifier ("you control") alike. The term does not mark
  which a phrase is.
- **Membership is by kind.** A position sort admits every phrase of a member kind, including
  wordings English does not print there: "is other", "other than X creature", "legendary" after a
  head. No finer sort refuses them; semantics rejects what means nothing.

**`Test` (NEW) holds the unspelled tests.** About thirty primitives say what a describing phrase
means and print nothing: `hasPossessor`, `inCombat`, `compare`, `shares`, `superlative`,
`colorCount`, `currentFace`, `hasDesignation`. Today each is copied per position sort, the
Templates that carry the wordings are written over them ("you control" is `hasPossessor(…)`,
"with power 3 or less" is `compare(…)`, "attacking" is `inCombat(…)`), and rule bodies and 44
cards also call them bare. A test is no kind of phrase, so it is declared once, at `Test`:

- A declaration at `Test` has no `spelling`; one that has is refused at load. So recovery never
  yields a test, and a test is never a second written form of a phrase.
- `Test` is a member of the three position sorts directly, beside the kinds, so a bare test still
  stands wherever it is called today.
- **A Template at a kind may have its body at `Test`**, declared per kind:

      expands(AdjectivePhrase): Test
      expands(PrepositionPhrase): Test
      expands(RelativeClause): Test
      expands(Participle): Test

  The body resolves at the Template's own sort when its root word stands in that sort's Reaches,
  and otherwise at `Test`. The choice is never open: the kind and `Test` reach one position sort,
  so §3 already refuses a word declared at both. A parameter typed by a kind must also admit a
  `Test`, since expansion puts one there; none is typed so today, and load refuses one that is.
- `Test` has `coord` and `not` of its own, and unspelled `hasType`, `hasSubtype`, `hasSupertype`
  and `hasColor`, for bodies that combine tests: `historic` is `coord(or, [hasType(artifact),
  hasSupertype(legendary), hasSubtype(saga)])`, an adjective phrase that stands before a head and
  after a copula (*Curator's Ward*'s "if it was historic", *Desynchronization*'s "that's not
  historic").

A card that calls a test bare writes something it does not print: *Blessed Reversal*'s "creature
attacking you" is `inCombat(attackerOf, counterpart = you)` today. Each such site takes the
spelled phrase once one exists (§4), and `syl-spelled-test-phrases` writes the rest and then
refuses a test in a card's term.

**`Participle` (NEW) holds a participle that modifies a head.** "Attacking", "blocked",
"enchanted", "named X", "attached to it" and "controlled by X" are verb phrases in their
participle form (*Smite*'s "target blocked creature" parses so). Until the passive is built each
is a fixed Template over a test, and those Templates stand at `Participle`, a member of
`PreModifier` and `PostModifier` and not of `Predicative`, where "be" with a participle is the
passive. One `attacking(defender = none)` then serves "attacking creature" and "creature
attacking you". `syl-passive-participle-and-trigger-events` replaces each fixed participle with
its verb and `Participle` with `VerbPhrase` in the two memberships, stating the form on the
membership (`lexical = {form = Participle}`: the gerund-participle of an active phrase, the past
participle of a `done` phrase with no "be"), so a Hole typed by the position sort takes no form
label (`spelling.md` §7). It also renames one of each pair `VerbPhrase` then collides with, `copy`
(a `Nominal` and a verb) and `attachment` (a verb and a test). Where a status adjective and a
passive participle print one word ("tapped" and the participle of `tap`), the written form is
the adjective, which is how the grammar reads the bare word.

What a position sort does not hold: the passive and the progressive ("is attached to it", "isn't
being declared as an attacker", "that's attacking you") are verb phrases under "be"
(`docs/specs/voice-and-object-phrases.md` §3), and a "has …" clause is a verb phrase headed by
"have".

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
- `Color` and `Supertype` are members of `AdjectivePhrase`. "Isn't legendary" (74 units, *Yenna,
  Redtooth Regent*) has a term, `is(…, not(legendary))`, which no coercion gave it.
- `Int` is a member of `Amount`, and `lit` goes.
- `Status` is a member of nothing. It stays the sort of the game's statuses, which `setStatus`,
  `becomeStatus`, `entersAs` and the test `hasStatus` take. Its words are not one kind of phrase,
  so each wording is a Template at its kind over `hasStatus`: `tapped`, `untapped` and `flipped`
  at `AdjectivePhrase`; `faceDown`, `faceUp`, `phasedIn` and `phasedOut` at `PrepositionPhrase`,
  spelled "face down", "face up", "phased in" and "phased out". The hyphenated "face-down" and
  "phased-out" stand only before a head (60 units and 1) and stay the `PreModifier` Templates of
  those names. A status word and the Template of its name share no Reaches set.
- The words of the wrappers' names at `Predicate` (`hasType`, `hasColor` and the rest) and
  `subtype` at the sorts a text change and a choice read are not wrappers and stay.

## 3. Resolution

`syl.md` §3's order keeps four steps, and its fourth changes. At expected sort `T`, a value
resolves:

1. `none`, if `T` is optional;
2. what stands at `T` itself;
3. if `T` is `Option(U)`, the value at `U`, by steps 2 and 4;
4. at the one sort among `T`'s members, and theirs, that the value stands at.

**Reaches.** Reaches(`T`) is `T` and every sort that is a member of it, directly or through a
member. Membership has no cycle and no sort reaches another by two paths; load refuses either.
The load rules of `syl.md` §3 hold over Reaches unchanged, with the exception below: a word
declared at two sorts of Reaches(`T`) is refused, and so is a literal that stands at two (an
integer when `Int` and `Nat` both reach `T`). So step 4 finds at most one sort, with no
backtracking. The side table that records the sort each call resolved at already says where the
value stands. No word collides under §2's memberships once §4 is done
(`docs/evidence/syl-kind-sorts/test-sort-check.md` has the computation).

**A position sort's own declarations.** A declaration at `PreModifier`, `PostModifier` or
`Predicative` is one of two things:

- a word from the sort to itself: a primitive or Template with a parameter of that sort, or a list
  of it (coordination `or`, `andOr`, `and`, `both`, `coord`; negation `not`; the prefix `non-`);
- a Template for a wording bound to that position ("face-down" before a head; `withMostVotes`,
  "with the most votes or tied for most votes", a coordination of unlike kinds fixed in one
  wording).

A word of the first kind is found at the position before any member is tried (step 2 comes
before step 4), so members' words of the same name are out of reach there and do not collide,
with it or with each other: `or([white, blue])` in `pre` is `PreModifier`'s `or`, though `Nominal`
has an `or` too, and `coord` stands at `PreModifier`, `Nominal` and `Test`. That is the one
written form of a coordination at a position, whether its arms are of one kind or several. The
grammar parses a coordination of like kinds at the kind ("white or blue" is one coordination of
adjective phrases) and only unlike kinds at the function, so recovery maps both to the position
sort's coordinator, and a coordinated `Nominal` is refused in `pre` as a nested one is. A
coordinator is still declared once per sort it coordinates.

Anything else at a position sort is refused at load, once the declarations §4 lists as staying
there have left; the ticket that removes the last of them adds the refusal.

**A nominal in `pre`** is a bare head word (`creature`, `goblin`, `token`). `nominal(…)` with a
`pre` or `post` of its own is refused there, since `nominal(pre = [nominal(pre = [legendary],
creature)], card)` and `nominal(pre = [legendary, creature], card)` would be two written forms of
"legendary creature card". The one place it stands is as an arm of a coordination in `pre`:
*Acclaimed Contender*'s "Knight, Aura, Equipment, or legendary artifact card" has
`nominal(pre = [legendary], artifact)` as its last arm, and `stacked` goes. The checker's rules
on a nominal's lists that are keyed on a position sort's test (`("PostModifier", "Agree")`,
`("PostModifier", "Superlative")` and the others in `crates/xtask/src/dsl_check/nominal.rs`)
follow the test to `Test`.

**Spelling skeletons** (`spelling.md` §7). Two sorts overlap when their Reaches sets intersect, so
a Hole typed `PreModifier` overlaps one typed `AdjectivePhrase`. Two declarations whose sorts lie
in one sort's Reaches and whose skeletons are the same are refused as ambiguous, as two at one
sort are today; a position sort's coordinator and a member's of the same name are one wording
there, written as the position sort's.

## 4. What each declaration becomes

Every declaration at `PreModifier`, `PostModifier` and `Predicative` in `plugins/builtin/`: 135.
Fifty move to a kind, 19 are the tests declared once at `Test`, 39 are deleted, 18 are the
position sort's own and 9 stay until another ticket composes them. The placement is computed from
the checker's own resolutions (`docs/evidence/syl-kind-sorts/test-sort-check.md`): every caller
still resolves, and every moved Template's body resolves at its kind or at `Test`.

### Moves to its kind

| Kind | Declarations | Decided by |
|---|---|---|
| `AdjectivePhrase` | `other`, `otherThan`, `split`, `colorless`, `multicolored`, `historic`, `unblocked`, `transformed`, `tiedFor`; the new `tapped`, `untapped`, `flipped`; the color and supertype words by membership | "red" 496 before a head, 21 after a copula (*Fry*); "legendary" 334 / 74; "colorless" 539 / 15; "multicolored" 64 / 1; "historic" 60 / 2; "unblocked" 13 / 4; "other" 1,567 before a head; "other than" 167 after a head; "tied" 34 after a copula; "tapped" 353 / 230 |
| `PrepositionPhrase` | `in`, `inPile`, `of`, `fromSource`, `ofChosen`, `ofYourChoice`, `ofTheLastChosen`, `hasPower`, `hasManaValue`, `hasLesserManaValueThan`, `activationCostHas`, `manaCostHas`, `tapOrUntapInCost`, `withDifferentNames`, `withTheSameName`; `withCounter` and `withKeyword` (below); the new `faceDown`, `faceUp`, `phasedIn`, `phasedOut` | "in" 1,624 after a head, 313 after a copula (*Animate Dead*); *Aerial Volley* "with flying"; *Abrupt Decay* "with mana value 3 or less"; *Cursed Totem* "of creatures"; *Gaea's Revenge* "from nongreen sources"; *Adaptive Automaton* and *Crippling Fear* "of the chosen type"; *Unable to Scream* "is face down" |
| `RelativeClause` | `youControl`, `youDontControl`, `youOwn`, `opponentsControl`, `anOpponentControls`, `controls` (below), `ownedBy`, `actorControls`; `thatIs`, `couldEnchant`, `triggeredNotLeftStack`; `did` | *Glorious Anthem* "you control"; *Academy Journeymage* "an opponent controls"; *Fry* "that's white or blue"; *Bruna, Light of Alabaster* "that could enchant it" |
| `Participle` | `attacking`, `blocking`, `blocked`, `chosen`, `equipped`, `foretold`, `plotted`, `copied`, `attachedTo`, `named`, `notChosen`; `was`, one Template for its `PreModifier` and `PostModifier` copies; the new `controlledBy` ("controlled by $who") | *Smite* "blocked", *Stone Haven Outfitter* "Equipped", *Bioplasm* "exiled"; "attach" passive 178 after a head; "name" passive 333; *Burn at the Stake* "tapped this way"; *Balance* "lands controlled by the player who controls the fewest" |

- **Three renames, each one name for two wordings today.** `controlledBy` at `PostModifier`
  spells "$who control" and at `Predicative` "controlled by $who": the relative is `controls`,
  and `controlledBy` names the participle. `hasCounter` and `hasKeyword` at `PostModifier` spell
  "with a counter on it" and "with $keyword" and at `Predicative` are the unspelled tests: the
  with-phrases are `withCounter` and `withKeyword`, Templates over the tests of the old names.
- **Bodies rewritten.** `historic` (§2); `youDontControl` is `not(hasPossessor(controller, you))`;
  `was` is written over `predicateDid`, the test `did` is over; `tapOrUntapInCost` is an `or` of
  two `activationCostHas` phrases, with `or` declared at `PrepositionPhrase`.
- **Cards that take the spelled phrase.** *Blessed Reversal*, *Tidal Flats* and the other cards
  that print "attacking X" or "blocking X" after a head write `attacking(…)` and `blocking(…)`
  where they call `inCombat` today. *Propaganda*, *Ghostly Prison* and *Awesome Presence* print a
  relative, "that's attacking you", and keep the test until the passive ticket.

### Declared once at `Test`

`agree`, `attachment`, `choseExtreme`, `chosenRef`, `colorCount`, `compare`, `currentFace`,
`hasCounter`, `hasDesignation`, `hasKeyword`, `hasPossessor`, `hasStatus`, `inCombat`,
`legalAttachment` (renamed, since a `Nominal` has the name), `predicateDid`, `related`, `shares`,
`superlative`, `targeting`, with the new `coord`, `not`, `hasType`, `hasSubtype`, `hasSupertype`
and `hasColor`. Each replaces its copies at the position sorts. Every copy of a test builds one
constructor, and where signatures differ the longer one has the shorter's parameters with the
rest optional (`inCombat`, `hasDesignation`, and `attachment` with `word` passed by name), so
every call resolves unchanged.

### Deleted

- The nine coercions, and the wrappers `hasColor`, `hasSupertype`, `hasType` and `hasSubtype` at
  `PreModifier`, `hasColor` and `hasStatus` at `Predicative`, `type`, `subtype` and `lit`.
- The copies of each test at a second and third position sort (fifteen).
- `in`, `ofChosen`, `otherThan` and `transformed` at `Predicative`: the declaration that moves,
  now reached by membership (*Animate Dead*, *Crippling Fear*).
- `token`, `permanent` and `outlaw` at `PreModifier`: the `Nominal` word (§2). `tapped` and
  `untapped` at `PreModifier`: the adjective phrase.
- `noun` at `Predicative`: *Archon of Coronation*'s "you're the monarch" is the noun phrase
  itself.
- `stacked` (§3), and the `PreModifier` copy of `was`.
- `not` at `PostModifier`. Each of its callers writes what it prints: a `without` preposition
  phrase ("without flying"), "with no …" as its own wording, a relative clause over the
  predicative `not` ("that aren't legendary"), or `not` inside the participle (*Escaped
  Shapeshifter*'s "not named …"). Two calls in `soulbond` negate a test and take `Test`'s `not`.

### The position sort's own

- `PreModifier`: `or`, `andOr`, `coord`, `non`, `not`; the Templates bound to the place before a
  head, `faceDown` ("face-down") and `phasedOut` ("phased-out").
- `PostModifier`: `or`, `andOr`, `coord`; `withMostVotes` (*Council's Judgment*).
- `Predicative`: `and`, `or`, `andOr`, `both`, `coord`, `not`; `hasTypeIgnoringTypeChanges`
  (sunburst's rule).

### Stays until another ticket composes it

| Declarations | Prints | Ticket |
|---|---|---|
| `attachedTo`, `attachedBy`, `controlledBy`, `named`, `was` at `Predicative` | "are already attached to it" (*Benevolent Blessing*), "was cast from your hand" (*Approach of the Second Sun*): the passive, `be(done(…))` | `syl-passive-participle-and-trigger-events` |
| `did` at `Predicative` | "if you cast it" (*Arcane Proxy*): `did(phrase, within)` under `holds` | `syl-lookback-operator-and-holds` |
| `outsideTheGame` at `PostModifier` and `Predicative` | "from outside the game" (19 of 22 units): a place | `syl-zone-words` |
| `hasQuality` at `PreModifier` | 23 rule bodies put a keyword's quality in `pre` through it | `syl-keyword-quality-union` (NEW) |

The refusal of §3 waits for the last of them.

### `Predicate`

This spec leaves `Predicate` and its 36 declarations as they are. Thirteen keywords take their
quality at it (`protection`, `landwalk`, `affinity`, `enchant` and nine more), and about twenty
other parameters take one (`behold`, `transform`'s `into`, `resulting` on `cast`, `except` on
`allExcept`, `holders` on `keywordGrant`, `matching`, `oneEachOf` and others). `Test` is not a
member of it: `Predicate` holds spelled copies of `hasType`, `in`, `youControl` and others that
would collide. `syl-keyword-quality-union` decides what a keyword's quality is typed by and moves
what is left of `Predicate`'s tests to `Test`.

## 5. Kinds the pool prints that this spec does not place

| Position | Phrase | Units |
|---|---|---|
| before a head | a keyword ("Equip abilities", "flashback cost") | 163 |
| before a head | a power/toughness measure outside tokens and counters ("Target 1/1 creature", *Aegis of the Meek*) | 12 |
| before a head | a name ("Wicked Role token") | 69 |
| before a head | mana symbols ("{2}{U} kicker") | 30 |
| after a head | a temporal noun phrase ("the first time each turn") | 131 |
| after a copula | a measure ("except it's 1/1", *Endless Evil*) | 23 |

A counter's kind before "counter" is `syl-counters-noun-phrase`'s. The rest are
`syl-unplaced-describing-kinds` (NEW).

## 6. Tickets

Built by:

1. **`syl-union-membership`** (NEW): `member` and `expands` in `sylex_syl` and `cargo xtask dsl
   check`, with coercion removed; the sorts of §1 and §2; §4's moves, tests, deletions and
   renames. It needs none of the verb tickets.
2. **`syl-spelled-test-phrases`** (NEW): the wordings cards print where they call a test bare
   (`shares`, `superlative`, `targeting`, `compare`, the designations), and the refusal of a test
   in a card's term.
3. **`syl-keyword-quality-union`** (NEW), **`syl-have-verb-phrases`** (NEW) and
   **`syl-unplaced-describing-kinds`** (NEW).

Changed by this spec: `syl-passive-participle-and-trigger-events` (the fixed participles become
their verbs, `VerbPhrase` replaces `Participle`, and the two renames of §2),
`syl-lookback-operator-and-holds` and `syl-clause-and-imperative` (`did` and `was`; a relative
composed from a clause with a gap, at `RelativeClause`), `syl-zone-words` (a location phrase is a
`PrepositionPhrase`), `syl-damage-and-life-noun-phrases` (its modifiers are declared at kinds),
`syl-become-replaces-types` and `syl-token-records-to-templates` (they consume `Predicative`),
`syl-canon-wording-defects` (`copied`, `choseExtreme`), `dsl-fmt-bundle-normalization` (no
coercion is left to normalize), and `syl-name-originally-printed-in`,
`syl-daybound-reconfigure-bodies` and `syl-clone-any-on-the-battlefield` (each asks for a new word
at `Predicate`, which is declared at its kind instead). `syl-verb-phrase-dependents` builds a
depictive ("enters tapped", "exile it face down") from the same kinds; `MoveRider` stays until it
does.

## 7. Not decided here

- Declaring a coordinator once for every sort it coordinates.
- A typed Rust representation of the surface term, in which a sort with members is an enum over
  them (`PreModifier::Adjective(Adjective::Color(Color::Red))`, with one `Test` arm per position
  sort).

## 8. What the design docs say once this is built

| Document | Today | Then |
|---|---|---|
| `syl.md` §2 | "No conversions besides `Option` and the declared coercions"; "There are nine" | no conversions; the `member` and `expands` declarations and §2's memberships |
| `syl.md` §4 | a Template's body is a term of the Template's sort | or of `Test`, where its kind `expands` to it |
| `syl.md` §3 | step 4 is a coercion, one hop; Reaches(`T`) is `T` and the from-sorts of coercions into it | §3 here |
| `surface.md` §1 | "`Nominal` and its modifier sorts"; "the sorts of the words a coercion wraps (`CardType` under `Nominal`)" | the kind sorts of §1; a position sort names a place kinds stand; a word sort is a member of its kind |
| `spelling.md` §2 | `stacked` for an arm of two premodifiers; `thatIs(predicative)` at `PostModifier`; a color at `Predicative` "through its coercion" | a nominal arm (§3); `thatIs` at `RelativeClause`; membership |
| `spelling.md` §4 | the bare word a coercion wraps and the written call are two written forms, and recovery yields the bare one | one written form |
| `spelling.md` §7 | "A sort is not one Category: a `PreModifier` is a noun in 'creature card' and an adjective in 'tapped creature'" | a kind sort is one Category and a position sort a function; a membership's `form`; the skeleton rule of §3 |
| `semantics.md` §2.4 | "at `PreModifier` it tests the characteristic alone"; "`Predicative`'s noun-phrase form `noun(phrase)`" | the type word's reading is positional (§2); a noun phrase is a member of `Predicative` |

Glossary entries (`docs/contexts/game-model/CONTEXT.md`):

- **Coercion**: deleted. **Membership** (NEW): a sort's values are also values of each sort it is
  a member of.
- **PreModifier**, **PostModifier**, **Predicative**: each is a position sort, defined by its
  members and the place it types.
- **Test** (NEW): an unspelled primitive that says what a describing phrase means; **Predicate**
  keeps its entry until its last consumer leaves.
