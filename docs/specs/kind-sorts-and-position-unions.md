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
| `PreModifier` | `AdjectivePhrase`, `Nominal`, `VerbPhrase` (as a participle) |
| `PostModifier` | `PrepositionPhrase`, `RelativeClause`, `VerbPhrase` (as a participle), `AdjectivePhrase` |
| `Predicative` | `AdjectivePhrase`, `PrepositionPhrase`, `NounPhrase` |
| `AdjectivePhrase` | `Color`, `Supertype` |
| `Nominal` | `CardType`, `Subtype` |
| `Amount` | `Int` |

- `PreModifier` and `PostModifier` type the `pre` and `post` lists of `nominal`, and `Predicative`
  the complement of `is`, `become`, `remain` and `turn`, as today. They are the **position
  sorts**: each names a place, and its members are the kinds that stand there.
- `post` holds every dependent printed after the head, in printed order: a complement the head
  selects ("half of this split card") and a modifier ("you control") alike. The term does not mark
  which a phrase is.
- **A membership may state a verb form.** `lexical = {form = Participle}` on `VerbPhrase`'s
  membership says a verb phrase there is a participle, so a Hole typed by the position sort takes
  no form label and its verb is governed as it is after "can't" (`spelling.md` §7). Which
  participle is the voice's: an active verb phrase prints its gerund-participle ("attacking",
  "blocking it"), and a `done` phrase its past participle with no "be" ("exiled", "dealt damage by
  this creature"). A Template that stands for a passive participle before `done` exists (`was`)
  labels its own Hole `@PastParticiple`.
- **Membership is by kind.** A position sort admits every phrase of a member kind, including
  wordings English does not print there: "is other", "other than X creature", "legendary" after a
  head. No finer sort refuses them; semantics rejects what means nothing.

What a position sort does not hold: the passive and the progressive ("is attached to it", "isn't
being declared as an attacker") are verb phrases under "be"
(`docs/specs/voice-and-object-phrases.md` §3), not members of `Predicative`, and a "has …" clause
is a verb phrase headed by "have".

**What the nine coercions become.** Each wrapped a word through a primitive (`hasColor($color)`,
`type($type)`, `lit($value)`); the word is now a member and the primitive goes, so the bare word
is the only written form.

- `CardType` and `Subtype` are members of `Nominal`, and `Nominal` of `PreModifier`, so a type or
  subtype word is one word wherever it stands: "creature" in "creature card" is the word the head
  "creature" is. `token`, `permanent` and `outlaw` are each one declaration as head and as
  modifier. That a description with a type word, no zone and no "card", "spell" or "source" means
  a permanent on the battlefield [CR#109.2] is read by semantics from where the word stands; the
  term does not write it. `type`, `subtype`, `hasType` and `hasSubtype` go.
- `Color` and `Supertype` are members of `AdjectivePhrase`. "Isn't legendary" (74 units, *Yenna,
  Redtooth Regent*) has a term, `is(…, not(legendary))`, which no coercion gave it. `hasColor` and
  `hasSupertype` go.
- `Int` is a member of `Amount`, and `lit` goes.
- `Status` is not a sort of the surface term. Its words are not one kind of phrase, and each is
  declared at its own: `tapped`, `untapped` and `flipped` at `AdjectivePhrase`; `faceDown`,
  `faceUp`, `phasedIn` and `phasedOut` at `PrepositionPhrase`, spelled "face down", "face up",
  "phased in" and "phased out" as today. The hyphenated "face-down" and "phased-out" stand only
  before a head (60 units and 1) and stay the `PreModifier` Templates of those names. Each word
  lowers to the one status the game has. `becomeStatus` is `become` with its `Predicative`
  complement, `entersAs` keeps its rider until `syl-verb-phrase-dependents`, and `hasStatus` and
  the `PreModifier` Templates `tapped` and `untapped` go.

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

One pair collides today: `copy` is a `Nominal` ("a copy") and a `VerbPhrase` ("copy target
spell"), and both reach `PreModifier`. The ticket that builds membership renames one.

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

Every declaration at `PreModifier`, `PostModifier` and `Predicative` in `plugins/builtin/`, with
the card whose parse decides it. Where a row names no card, the declaration is called only from
rule bodies and no Reading was checked.

### One declaration at a kind

| Kind | Declarations | Decided by |
|---|---|---|
| `AdjectivePhrase` | `colorless`, `multicolored`, `historic`, `unblocked`, `transformed`; `other`; `otherThan`; `tiedFor`; `split`; the status adjectives of §2; the color and supertype words by membership | "red" 496 before a head, 21 after a copula (*Fry*); "legendary" 334 / 74; "colorless" 539 / 15; "multicolored" 64 / 1; "historic" 60 / 2; "unblocked" 13 / 4; "other" 1,567 before a head; "other than" 167 after a head; "tied" 34 after a copula (its three callers print it after "are"); `split` in aftermath's rule only |
| `Nominal` | `token`, `permanent`, `outlaw` (each already a head); the type and subtype words by membership | "token" 350, "permanent" 355, "outlaw" 10 before a head |
| `VerbPhrase` | `attacking`, `blocking`, `blocked`, `chosen`, `equipped`, `foretold`, `plotted`; `attachedTo`, `named`; `was`, one Template for its `PreModifier` and `PostModifier` copies, spelled "$action@PastParticiple< $within>" | *Smite* "blocked", *Stone Haven Outfitter* "Equipped", *Bioplasm* "exiled"; "attach" passive 178 after a head; "name" passive 333 after a head; *Burn at the Stake* "tapped this way"; `foretold` and `plotted` in rule bodies only |
| `PrepositionPhrase` | `in`, `inPile`; `hasKeyword` ("with $keyword"), `hasCounter`, `hasPower`, `hasManaValue`, `hasLesserManaValueThan`, `activationCostHas`, `manaCostHas`, `tapOrUntapInCost`, `withDifferentNames`, `withTheSameName`; `of`, `fromSource`, `ofTheLastChosen`, `ofChosen`, `ofYourChoice`; the status prepositions of §2 | "in" 1,624 after a head, 313 after a copula (*Animate Dead*); *Aerial Volley* "with flying"; *Abrupt Decay* "with mana value 3 or less"; *Cursed Totem* "of creatures"; *Gaea's Revenge* "from nongreen sources"; *Adaptive Automaton* and *Crippling Fear* "of the chosen type"; *Unable to Scream* "is face down" |
| `RelativeClause` | `youControl`, `youDontControl`, `youOwn`, `opponentsControl`, `anOpponentControls`, `controlledBy` ("$who control"), `ownedBy`, `actorControls`; `thatIs`, `couldEnchant`, `triggeredNotLeftStack`; `did` at `PostModifier` | *Glorious Anthem* "you control"; *Academy Journeymage* "an opponent controls"; *Fry* "that's white or blue"; *Bruna, Light of Alabaster* "that could enchant it"; `triggeredNotLeftStack` in rule bodies only |

The participle Templates at `VerbPhrase` are the verb phrases of
`docs/specs/voice-and-object-phrases.md` standing in `pre` and `post`:
`syl-passive-participle-and-trigger-events` replaces each fixed participle with its verb, and
`syl-lookback-operator-and-holds` replaces `did` and `was`. `did` at `PostModifier` spells "that
$subj $action" where *Alabaster Leech* prints the bare "you cast"; the lookback ticket writes
what the card prints.

### Deleted

- As duplicates of the `pre` or `post` declaration, now reached by membership: `hasColor`, `in`,
  `ofChosen`, `otherThan` and `transformed` at `Predicative` (*Fry*, *Battlegate Mimic*, *Animate
  Dead*, *Crippling Fear*).
- As wrappers membership replaces: the nine coercions; `hasColor`, `hasSupertype`, `hasType` and
  `hasSubtype` at `PreModifier`; `hasStatus` at `PreModifier` and `Predicative` and the
  `PreModifier` Templates `tapped` and `untapped`; `noun` at `Predicative` (*Archon of
  Coronation*'s "you're the monarch" is the noun phrase itself); `stacked`.
- `not` at `PostModifier`. Its callers print other wordings, and each writes what it prints: a
  `without` preposition phrase ("without flying"), "with no …" as its own wording, a relative
  clause over the predicative `not` ("that aren't legendary"), or `not` inside the participle
  (*Escaped Shapeshifter*'s "not named …").

### Words of the position sort

- `PreModifier`: `or`, `andOr`, `coord`, `non`, `not`; the position-bound Templates `faceDown`
  ("face-down") and `phasedOut` ("phased-out").
- `PostModifier`: `or`, `andOr`, `coord`; the position-bound Template `withMostVotes` (*Council's
  Judgment*).
- `Predicative`: `and`, `or`, `andOr`, `both`, `coord`, `not`; the position-bound Template
  `hasTypeIgnoringTypeChanges` (sunburst's rule).

### Staying at the position sort until another ticket composes it

These keep the sort, name and signature they have today, one copy per position, and the refusal
of §3 waits for the last of them.

| Today | Prints | Becomes | Ticket |
|---|---|---|---|
| `attachedTo`, `attachedBy`, `controlledBy`, `was`, `named` at `Predicative` | "are already attached to it" (*Benevolent Blessing*), "was cast from your hand" (*Approach of the Second Sun*) | the passive, `be(done(…))` | `syl-passive-participle-and-trigger-events` |
| `inCombat` at `Predicative` | "isn't being declared as an attacker" (*Verity Circle*) | a progressive over a passive | `syl-passive-participle-and-trigger-events` |
| `notChosen` at `PostModifier` | "not chosen this way" (*Celestial Judgment*), "that weren't chosen this way" (*Disciple of Caelus Nin*) | `not` inside the participle; a relative over the passive | `syl-passive-participle-and-trigger-events` |
| `did` at `Predicative`; `predicateDid` at all three | "if you cast it" (*Arcane Proxy*) | `did(phrase, within)` under `holds` | `syl-lookback-operator-and-holds` |
| `hasCounter`, `hasKeyword`, `compare`, `shares` at `Predicative` | "has no ice counters on it" (*Dark Depths*), "didn't have defender" (*Bristlepack Sentry*), "has the same name as" (*Candles of Leng*) | a verb phrase headed by `have` | `syl-have-verb-phrases` (NEW) |
| `hasDesignation` at all three | "you're the monarch" (*Archon of Coronation*), "while saddled" (*Autarch Mammoth*), "has the city's blessing" (ascend) | a noun phrase, an adjective phrase and a `have` phrase, one wording each | `syl-have-verb-phrases` (NEW) |
| `named` at `Predicative` (2 canon callers) | "has the chosen name" | a `have` phrase | `syl-have-verb-phrases` (NEW) |
| `choseExtreme` at `PostModifier` and `Predicative` | "with the highest number", "If you are one of those players" (*Menacing Ogre*) | a preposition phrase; a noun phrase after the copula | `syl-canon-wording-defects` |
| `copied` at `PreModifier` | "the copy" (*Rings of Brighthearth*) | a noun phrase | `syl-canon-wording-defects` |
| `outsideTheGame` at `PostModifier` and `Predicative` | "from outside the game" (19 of 22 units) | a place, the complement of "from" | `syl-zone-words` |
| the unspelled tests `hasPossessor`, `inCombat`, `compare`, `shares`, `superlative`, `attachment`, `currentFace`, `related`, `legalAttachment`, `agree`, `targeting`, `colorCount`, `chosenRef` | each is a meaning its callers word at one kind or several: `hasPossessor` as "you control" (a relative) and "controlled by you" (a participle); `inCombat` as "attacking you" and "that's blocking it"; `shares` as "with the same name as" and "that share …" (90 units) | the wording, composed from its verb or preposition | the passive, lookback and `have` tickets, each for the wordings it composes; what is left when they land is listed by the last of them |

### `Predicate`

This spec leaves `Predicate` and its 36 declarations as they are. Thirteen keywords take their
quality at it (`protection`, `landwalk`, `affinity`, `enchant` and nine more), 23 rule bodies put
that quality in `pre` through `hasQuality`, and about twenty other parameters take one (`behold`,
`transform`'s `into`, `resulting` on `cast`, `except` on `allExcept`, `holders` on `keywordGrant`,
`matching`, `oneEachOf` and others). `syl-keyword-quality-union` decides what a keyword's quality
is typed by. `hasQuality` stays at `PreModifier` until it does.

## 5. Kinds the pool prints that this spec does not place

| Position | Phrase | Units | Where it goes |
|---|---|---|---|
| before a head | a keyword ("Equip abilities", "flashback cost") | 163 | with a counter's kind before "counter": `syl-counters-noun-phrase` adds the keyword to `PreModifier` |
| before a head | a power/toughness measure ("Target 1/1 creature", *Aegis of the Meek*) | 12 outside tokens and counters | `syl-counters-noun-phrase` and `syl-token-records-to-templates`, with the measure's sort from `syl-pt-change-sort` |
| before a head | a name ("Wicked Role token") | 69 | `syl-token-records-to-templates` |
| before a head | mana symbols ("{2}{U} kicker") | 30 | not owned |
| after a head | a temporal noun phrase ("the first time each turn") | 131 | not owned |
| after a copula | a measure ("except it's 1/1", *Endless Evil*) | 23 | `syl-become-replaces-types` |

## 6. Tickets

Built by, in order:

1. **`syl-union-membership`** (NEW): the `member` declaration, step 4 and Reaches in `cargo xtask
   dsl check`, with coercion removed; the three kind sorts; §2's memberships; every "one
   declaration at a kind", "deleted" and "words of the position sort" row of §4, with canon
   rewritten where a name or form changes. It declares `RelativeClause` and moves today's fixed
   relatives onto it.
2. **`syl-have-verb-phrases`** (NEW): the "has …" wordings.
3. **`syl-keyword-quality-union`** (NEW): what a keyword's quality is typed by, and `hasQuality`.

Changed by this spec: `syl-zone-words` (a location phrase is a `PrepositionPhrase`),
`syl-damage-and-life-noun-phrases` (its modifiers are declared at kinds),
`syl-passive-participle-and-trigger-events`, `syl-lookback-operator-and-holds` and
`syl-clause-and-imperative` (the participle, `did` and `was`, and a relative composed from a
clause with a gap, each at its kind), `syl-become-replaces-types` and
`syl-token-records-to-templates` (they consume `Predicative`), `syl-canon-wording-defects`
(`copied`, `choseExtreme`), `dsl-fmt-bundle-normalization` (no coercion is left to normalize), and
`syl-name-originally-printed-in`, `syl-daybound-reconfigure-bodies` and
`syl-clone-any-on-the-battlefield` (each asks for a new word at `Predicate`, which is declared at
its kind instead). `syl-verb-phrase-dependents` builds a depictive ("enters tapped", "exile it
face down") from the same kinds; `MoveRider` stays until it does.

## 7. Not decided here

- Declaring a coordinator once for every sort it coordinates.
- A typed Rust representation of the surface term, in which a sort with members is an enum over
  them (`PreModifier::Adjective(Adjective::Color(Color::Red))`).

## 8. What the design docs say once this is built

| Document | Today | Then |
|---|---|---|
| `syl.md` §2 | "No conversions besides `Option` and the declared coercions"; "There are nine" | no conversions; a `member` declaration and §2's memberships |
| `syl.md` §3 | step 4 is a coercion, one hop; Reaches(`T`) is `T` and the from-sorts of coercions into it; `tapped` at `Status` and at `PreModifier` is no collision | §3 here |
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
