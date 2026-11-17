# Kind sorts and membership: a describing phrase is declared once, at its kind of phrase

A phrase that describes something is declared at the sort that names its kind: an adjective
phrase, a preposition phrase, a relative clause, a verb phrase, a nominal or a noun phrase. The
places such a phrase stands (before a head, after it, after a copula) are sorts whose members are
those kinds, so one declaration serves every place its kind is admitted. Membership replaces
coercion: a color word is a member of the adjective phrases as an adjective phrase is a member of
the premodifiers, and nothing wraps either.

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

## 2. Membership

**A sort may have member sorts.** Every value of a member is a value of the sort, written as
itself: membership has no body and builds nothing. It is transitive, and it is the only relation
between sorts besides `Option` and `List`; there are no coercions.

A membership is a declaration with no name, in `plugins/builtin/members/`:

    member Color: AdjectivePhrase
    member AdjectivePhrase: PreModifier
    member VerbPhrase: PreModifier {
      lexical = {form = Participle}
    }

The memberships:

| Sort | Members |
|---|---|
| `PreModifier` | `AdjectivePhrase`, `Nominal`; `VerbPhrase` (as a participle) once the passive is built |
| `PostModifier` | `PrepositionPhrase`, `RelativeClause`, `AdjectivePhrase`; `VerbPhrase` (as a participle) once the passive is built |
| `Predicative` | `AdjectivePhrase`, `PrepositionPhrase`, `NounPhrase`, `Status` |
| `AdjectivePhrase` | `Color`, `Supertype` |
| `Nominal` | `CardType`, `Subtype` |
| `Amount` | `Int` |

- `PreModifier` and `PostModifier` type the `pre` and `post` lists of `nominal`, and `Predicative`
  the complement of `is` and `thatIs`, as today. They are the **position sorts**: each names a
  place, and its members are the kinds that stand there.
- `post` holds every dependent printed after the head, in printed order: a complement the head
  selects ("half of this split card") and a modifier ("you control") alike. The term does not mark
  which a phrase is.
- **Membership is by kind.** A position sort admits every phrase of a member kind, including
  wordings English does not print there: "is other", "other than X creature", "legendary" after a
  head. No finer sort refuses them; semantics rejects what means nothing.
- **A verb phrase joins `PreModifier` and `PostModifier` with the passive.** Today a participle
  before or after a head is a fixed Template (`attacking`, `blocked`, `attachedTo`) over an
  unspelled test, and the verbs `attack` and `block` would print the same words if any verb
  phrase could stand there. `syl-passive-participle-and-trigger-events` replaces each fixed
  participle with its verb and declares the two memberships in the same change. The membership
  states the form, `lexical = {form = Participle}`: a verb phrase there is a participle, the
  gerund-participle of an active phrase ("attacking", "blocking it") and the past participle of a
  `done` phrase with no "be" ("exiled", "dealt damage by this creature"), so a Hole typed by the
  position sort takes no form label (`spelling.md` §7). `Participle` is a value of that key only,
  not a lexicon Word Form. Two names collide when `VerbPhrase` joins, `copy` (a `Nominal` and a
  verb) and `attachment` (a verb and an unspelled test), and that ticket renames one of each
  pair. Where a status adjective and a passive participle print one word ("tapped" and the
  participle of `tap`; "untapped", "transformed", "flipped"), the written form is the adjective,
  which is how the grammar reads the bare word.

What a position sort does not hold: the passive and the progressive ("is attached to it", "isn't
being declared as an attacker") are verb phrases under "be"
(`docs/specs/voice-and-object-phrases.md` §3), not members of `Predicative`, and a "has …" clause
is a verb phrase headed by "have".

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
  `Nominal` primitives `type` and `subtype` and the `PreModifier` primitives `hasType` and
  `hasSubtype` go.
- `Color` and `Supertype` are members of `AdjectivePhrase`. "Isn't legendary" (74 units, *Yenna,
  Redtooth Regent*) has a term, `is(…, not(legendary))`, which no coercion gave it. `hasColor` at
  `PreModifier` and `Predicative` and `hasSupertype` at `PreModifier` go.
- `Int` is a member of `Amount`, and `lit` goes.
- `Status` is a member of `Predicative`, so "is tapped" and "is face down" take the status word
  as they do now, and `hasStatus` at `Predicative` goes. `Status` keeps its words, their
  spellings and its other consumers (`setStatus`, `becomeStatus`, `entersAs`). Before a head a
  status is a Template bound to that position (`tapped`, `untapped`, and the hyphenated
  `faceDown` and `phasedOut`) over `hasStatus` at `PreModifier`, as today.
- The words of the same names at other sorts are not wrappers and stay: `hasType`, `hasColor`
  and the rest at `Predicate`, and `subtype` at the sorts a text change and a choice read.

## 3. Resolution

`syl.md` §3's order keeps four steps, and its fourth changes. At expected sort `T`, a value
resolves:

1. `none`, if `T` is optional;
2. what stands at `T` itself;
3. if `T` is `Option(U)`, the value at `U`, by steps 2 and 4;
4. at the one sort among `T`'s members, and theirs, that the value stands at.

**Reaches.** Reaches(`T`) is `T` and every sort that is a member of it, directly or through a
member. Membership has no cycle. The load rules of `syl.md` §3 hold over Reaches unchanged, with
the exception below: a word declared at two sorts of Reaches(`T`) is refused, and so is a literal
that stands at two (an integer when `Int` and `Nat` both reach `T`). So step 4 finds at most one
sort, with no backtracking, and since membership builds nothing, which chain of memberships joins
that sort to `T` changes nothing. The side table that records the sort each call resolved at
already says where the value stands.

No word collides under these memberships once §4's moves are made
(`docs/evidence/syl-kind-sorts/move-or-stay.md` has the computation).

**A position sort's own declarations.** A declaration at `PreModifier`, `PostModifier` or
`Predicative` is one of two things:

- a word from the sort to itself: a primitive or Template with a parameter of that sort, or a list
  of it (coordination `or`, `andOr`, `and`, `both`; negation `not`; the prefix `non-`);
- a Template for a wording bound to that position ("face-down" before a head; `withMostVotes`,
  "with the most votes or tied for most votes", a coordination of unlike kinds fixed in one
  wording).

A word of the first kind may share its name with a member's word, and at the position the name
resolves to the position sort's (step 2 comes before step 4): `or([white, blue])` in `pre` is
`PreModifier`'s `or`. That is the one written form of the coordination there, whether its arms are
of one kind or several, and the exception to the Reaches refusal. A coordinator is still declared
once per sort it coordinates.

Anything else at a position sort is refused at load, once the declarations §4 lists as staying
there have left; the ticket that removes the last of them adds the refusal.

**A nominal in `pre`** is a bare head word (`creature`, `goblin`, `token`). `nominal(…)` with a
`pre` or `post` of its own is refused there, since `nominal(pre = [nominal(pre = [legendary],
creature)], card)` and `nominal(pre = [legendary, creature], card)` would be two written forms of
"legendary creature card". The one place it stands is as an arm of a coordination in `pre`:
*Acclaimed Contender*'s "Knight, Aura, Equipment, or legendary artifact card" has
`nominal(pre = [legendary], artifact)` as its last arm, and `stacked` goes. The checker's other
rules on a nominal's lists (`crates/xtask/src/dsl_check/nominal.rs`) read the resolved sort of
each element as they do now.

**Spelling skeletons** (`spelling.md` §7). Two sorts overlap when their Reaches sets intersect, so
a Hole typed `PreModifier` overlaps one typed `AdjectivePhrase`. Two declarations whose sorts lie
in one sort's Reaches and whose skeletons are the same are refused as ambiguous, as two at one
sort are today; a position sort's coordinator and a member's of the same name are one wording
there, written as the position sort's.

## 4. What each declaration becomes

Every declaration at `PreModifier`, `PostModifier` and `Predicative` in `plugins/builtin/`: 135,
of which 22 move to a kind, 21 are deleted, 18 are the position sort's own and 74 stay where they
are. The split is computed from the checker's own resolutions under one rule: a declaration moves
to its kind only if everything its body calls resolves from that kind, every caller still
resolves, and no declaration of its name is left at a position sort the kind is a member of. A
Template's body resolves at the Template's own sort, and a position sort is never a member of a
kind, so a Template whose body calls a test that stays cannot move.

### Moves to its kind

| Kind | Declarations | Decided by |
|---|---|---|
| `AdjectivePhrase` | `other`, `otherThan`, `split`, `colorless`, `multicolored`, and the unspelled `colorCount` the last two are over | "other" 1,567 before a head; "other than" 167 after a head; "colorless" 539 before a head, 15 after a copula; "multicolored" 64 / 1; `split` in aftermath's rule only |
| `PrepositionPhrase` | `in`, `inPile`, `of`, `fromSource`, `ofChosen`, `ofYourChoice`, `ofTheLastChosen`, `activationCostHas`, `manaCostHas`, `withDifferentNames`, `withTheSameName`, and the unspelled `agree` the last two are over | "in" 1,624 after a head, 313 after a copula (*Animate Dead*); *Cursed Totem* "of creatures"; *Gaea's Revenge* "from nongreen sources"; *Adaptive Automaton* and *Crippling Fear* "of the chosen type" |
| `RelativeClause` | `thatIs`, `couldEnchant`, `triggeredNotLeftStack`, and the unspelled `legalAttachment` | *Fry* "that's white or blue"; *Bruna, Light of Alabaster* "that could enchant it"; `triggeredNotLeftStack` in rule bodies only |

The color, supertype, card type and subtype words reach their positions by membership.
`nominal.rs`'s Agreement rule is keyed on a `PostModifier` `agree` and follows it to
`PrepositionPhrase`.

### Deleted

- The nine coercions and the wrappers §2 names.
- `in`, `ofChosen` and `otherThan` at `Predicative`: the same declaration as the one that moves,
  now reached by membership (*Animate Dead*, *Crippling Fear*).
- `token`, `permanent` and `outlaw` at `PreModifier`: the `Nominal` word (§2).
- `noun` at `Predicative`: *Archon of Coronation*'s "you're the monarch" is the noun phrase
  itself.
- `stacked` (§3).
- `not` at `PostModifier`. Its callers print other wordings, and each writes what it prints: a
  `without` preposition phrase ("without flying"), "with no …" as its own wording, a relative
  clause over the predicative `not` ("that aren't legendary"), or `not` inside the participle
  (*Escaped Shapeshifter*'s "not named …").

Deleting `noun`, `not` and `stacked` rewrites 53 calls in 48 files; a moved word keeps its name,
so its callers do not change.

### The position sort's own

- `PreModifier`: `or`, `andOr`, `coord`, `non`, `not`; the Templates bound to the place before a
  head, `tapped`, `untapped`, `faceDown` ("face-down") and `phasedOut` ("phased-out"), over
  `hasStatus`.
- `PostModifier`: `or`, `andOr`, `coord`; `withMostVotes` (*Council's Judgment*).
- `Predicative`: `and`, `or`, `andOr`, `both`, `coord`, `not`; `hasTypeIgnoringTypeChanges`
  (sunburst's rule).

### Stays until another ticket composes it

Each keeps the sort, name and signature it has today, and the refusal of §3 waits for the last of
them.

| Declarations | Why it cannot move | Ticket |
|---|---|---|
| the fixed participles `attacking`, `blocking`, `blocked`, `chosen`, `equipped`, `foretold`, `plotted`, `attachedTo`, `named`, `notChosen`, `unblocked`; the passives at `Predicative` (`attachedTo`, `attachedBy`, `controlledBy`, `named`); the tests under them (`inCombat`, `attachment`, `chosenRef`, `hasDesignation` at `PreModifier`, `hasPossessor` at `Predicative`) | a participle is its verb, and "is attached to it" (*Benevolent Blessing*) the passive | `syl-passive-participle-and-trigger-events` |
| `did`, `was` and `predicateDid` at all three | "if you cast it" (*Arcane Proxy*), "tapped this way" (*Burn at the Stake*) are `did(phrase, within)` | `syl-lookback-operator-and-holds` |
| the relatives `youControl`, `youOwn`, `opponentsControl`, `anOpponentControls`, `controlledBy`, `ownedBy`, `actorControls`, and `hasPossessor` under them | `hasPossessor` is also called from rule bodies in other positions (`partyOf`), so the Templates over it cannot follow it to one kind | `syl-clause-and-imperative` (a relative composed from a clause with a gap) |
| `hasCounter`, `hasKeyword`, `shares` and `hasDesignation` at `PostModifier` and `Predicative`; `compare` at `Predicative` | the `Predicative` copies word "has no ice counters on it" (*Dark Depths*), "didn't have defender" (*Bristlepack Sentry*), "has the same name as" (*Candles of Leng*), and the same-named with-phrases cannot move while those copies stay | `syl-have-verb-phrases` (NEW) |
| `choseExtreme`, `copied` | their callers print other words (*Menacing Ogre*, *Rings of Brighthearth*) | `syl-canon-wording-defects` |
| `outsideTheGame` | "from outside the game" is a place | `syl-zone-words` |
| `hasQuality` | 23 rule bodies put a keyword's quality in `pre` through it | `syl-keyword-quality-union` (NEW) |
| `compare` at `PostModifier` with `hasPower`, `hasManaValue` and `hasLesserManaValueThan`; `superlative` with `tiedFor`; `currentFace` with `transformed`; `related`; `targeting` | each test is called directly from rule bodies and cards as well as through its Templates | `syl-unspelled-tests-in-descriptions` (NEW) |
| `historic`, `tapOrUntapInCost`, `youDontControl` | each body needs a coordinator or a negation at its kind (`or` at `AdjectivePhrase` over a mix of nominals and an adjective; `or` at `PrepositionPhrase`; `not` at `RelativeClause`) | `syl-unspelled-tests-in-descriptions` (NEW) |

### `Predicate`

This spec leaves `Predicate` and its 36 declarations as they are. Thirteen keywords take their
quality at it (`protection`, `landwalk`, `affinity`, `enchant` and nine more), and about twenty
other parameters take one (`behold`, `transform`'s `into`, `resulting` on `cast`, `except` on
`allExcept`, `holders` on `keywordGrant`, `matching`, `oneEachOf` and others).
`syl-keyword-quality-union` decides what a keyword's quality is typed by.

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

1. **`syl-union-membership`** (NEW): the `member` declaration, step 4 and Reaches in `cargo xtask
   dsl check`, with coercion removed; the three kind sorts; §2's memberships; §4's moves and
   deletions. It declares `RelativeClause`.
2. **`syl-have-verb-phrases`** (NEW), **`syl-keyword-quality-union`** (NEW),
   **`syl-unspelled-tests-in-descriptions`** (NEW) and **`syl-unplaced-describing-kinds`** (NEW),
   each after it.

Changed by this spec: `syl-passive-participle-and-trigger-events` (the participles, the
`VerbPhrase` memberships and the two renames of §2), `syl-lookback-operator-and-holds` and
`syl-clause-and-imperative` (`did` and `was`; a relative composed from a clause with a gap, at
`RelativeClause`), `syl-zone-words` (a location phrase is a `PrepositionPhrase`),
`syl-damage-and-life-noun-phrases` (its modifiers are declared at kinds),
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
  them (`PreModifier::Adjective(Adjective::Color(Color::Red))`).

## 8. What the design docs say once this is built

| Document | Today | Then |
|---|---|---|
| `syl.md` §2 | "No conversions besides `Option` and the declared coercions"; "There are nine" | no conversions; a `member` declaration and §2's memberships |
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
