# Kind sorts and position unions: a modifier is declared once, at its kind of phrase

A phrase that describes something is declared at the sort that names its kind: an adjective
phrase, a preposition phrase, a relative clause, a verb phrase, a nominal or a noun phrase. The
places such a phrase stands (before a head, after it, after a copula, as a keyword's quality) are
unions of those kinds, so one declaration serves every place its kind is admitted.

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
| `PrepositionPhrase` (NEW) | "in your graveyard", "with flying", "of the chosen type", "from nongreen sources" | after a head ("in": 1,624); after a copula for some ("in": 313) |
| `RelativeClause` (NEW) | "you control", "that's white or blue", "that could enchant it" | after a head only ("control" as a bare relative: 9,044) |
| `VerbPhrase` | as a participle: "attacking", "exiled", "named X", "attached to it" | bare before a head, with dependents after it; under "be" it is the passive or the progressive, not a predicative complement |
| `Nominal` | "creature", "Goblin", "token", "permanent" | before a head ("creature": 5,047), and as a head |
| `NounPhrase` | "the monarch", "a Kree" | after a copula |

A phrase is declared once, at its kind. "Become" and "remain" take the complement "be" takes
("tapped": 65 under be, 123 under become, 42 under remain).

## 2. Unions

A **union** is a sort whose values are the values of its member sorts. It is a relation between
sorts beside coercion: membership has no body and builds nothing, so the term at a union position
is the member's own term.

A union is declared in `plugins/builtin/unions/`, one file each, with no name of its own:

    union PreModifier = AdjectivePhrase | Nominal | VerbPhrase@GerundParticiple
    union PostModifier = PrepositionPhrase | RelativeClause | VerbPhrase@GerundParticiple | AdjectivePhrase
    union Predicative = AdjectivePhrase | PrepositionPhrase | NounPhrase
    union Quality = AdjectivePhrase | Nominal | NounPhrase

- `PreModifier` and `PostModifier` type the `pre` and `post` lists of `nominal`, and `Predicative`
  the complement of `is`, `become` and `remain`, as today.
- `Quality` (NEW) is a keyword's quality parameter: "protection from red" (an adjective phrase),
  "enchant creature" (a nominal), "protection from everything" (a noun phrase). The grammar reads
  these parameters as the same three kinds. Keywords declare their parameters as they do now.
- `post` holds every dependent printed after the head, in printed order: a complement the head
  selects ("half of this split card") and a modifier ("you control") alike. The term does not mark
  which a phrase is.
- **A member may state a verb form.** `VerbPhrase@GerundParticiple` says the form a verb phrase
  takes in that union, with the form labels of `spelling.md` §7: an active verb phrase prints its
  gerund-participle ("attacking", "blocking it"), and a `done` phrase prints its past participle
  with no "be" ("exiled", "dealt damage by this creature"), as the passive does wherever it
  stands. A Hole typed by the union therefore takes no form label, and a `VerbPhrase` Hole's verb
  is governed there as it is after "can't".
- **Membership is by kind.** A union admits every phrase of a member kind, including wordings
  English does not print there: "is other", "other than X creature", "is face-down", "charge
  creature". No finer sort refuses them; semantics rejects what means nothing.

What a union does not hold: the passive and the progressive ("is attached to it", "isn't being
declared as an attacker") are verb phrases under "be" (`docs/specs/voice-and-object-phrases.md`
§3), not members of `Predicative`; a "has …" clause is a verb phrase headed by "have" (§6).

## 3. Resolution

`syl.md` §3's order gains one step. At expected sort `T`, a value resolves:

1. `none`, if `T` is optional;
2. what stands at `T` itself;
3. if `T` is `Option(U)`, the value at `U`;
4. a coercion into `T`;
5. if `T` is a union, at the one member `M` the value stands at, by step 2 or step 4 at `M`.

A value takes at most one coercion and at most one membership step, in that order: `red` in `pre`
is a `Color`, coerced to `AdjectivePhrase`, a member of `PreModifier`. A coercion stays one hop,
and a union is never a member of a union or the from-sort of a coercion.

**Reaches.** For a union `T`, Reaches(`T`) is `T`, its members, and every from-sort of a coercion
into `T` or into a member. The load rules of `syl.md` §3 hold over it unchanged, with one
exception below: a word declared at two sorts of Reaches(`T`) is refused, and so is a literal that
stands at two. So step 5 finds at most one member, and resolution stays constant-time with no
backtracking. The side table that records the sort each call resolved at already says which
member was used.

**A union's own declarations.** A declaration at a union sort is one of two things:

- a word from the union to itself: a primitive or Template with a parameter of that union, or a
  list of it (coordination `or`, `andOr`, `and`, `both`; negation `not`; the prefix `non-`);
- a Template for a wording bound to that position (`withMostVotes`, "with the most votes or tied
  for most votes", a coordination of unlike kinds fixed in one wording).

Anything else declared at a union is refused at load. A word of the first kind may share its name
with a member's word, and at the union position the name resolves to the union's (step 2 comes
before step 5): `or([white, blue])` in `pre` is `PreModifier`'s `or`. That is the one written form
of the coordination there, whether its arms are of one kind or several, and the exception to the
Reaches refusal. A coordinator is still declared once per sort it coordinates.

**A nominal in `pre`** is a bare head word (`creature`, `goblin`, `token`). `nominal(…)` with a
`pre` or `post` of its own is refused there, since `nominal(pre = [nominal(pre = [legendary],
creature)], card)` and `nominal(pre = [legendary, creature], card)` would be two written forms of
"legendary creature card". The one place it stands is as an arm of a coordination in `pre`:
*Acclaimed Contender*'s "Knight, Aura, Equipment, or legendary artifact card" has
`nominal(pre = [legendary], artifact)` as its last arm, and `stacked` goes.

**Spelling skeletons** (`spelling.md` §7). Two sorts overlap when their Reaches sets intersect, so
a Hole typed `PreModifier` overlaps one typed `AdjectivePhrase`. Two declarations whose sorts lie
in one union's Reaches and whose skeletons are the same are refused as ambiguous, as two at one
sort are today; a union's own coordinator and a member's of the same name are one wording there,
written as the union's.

## 4. Coercions

Six remain:

- `CardType` and `Subtype` stand at `Nominal` (through `type` and `subtype`);
- `Color`, `Supertype` and `Status` stand at `AdjectivePhrase` (through `hasColor`, `hasSupertype`
  and `hasStatus`, each declared once, at `AdjectivePhrase`);
- `Int` stands at `Amount`.

The `CardType`, `Subtype`, `Supertype` and `Color` coercions into `PreModifier`, the `Color` and
`Status` coercions into `Predicative`, and `hasType` and `hasSubtype` go.

- **A type or subtype word is one word wherever it stands.** "Creature" in "creature card" is the
  `Nominal` the head "creature" is, reaching `pre` by membership, and `token`, `permanent` and
  `outlaw` are each one declaration as head and as modifier. That a description with a type word
  and no "card", "spell" or "source" means a permanent on the battlefield [CR#109.2] is read by
  semantics from where the word stands; the term does not write it.
- **A supertype word is an adjective**, so "isn't legendary" (74 units, *Yenna, Redtooth
  Regent*) has a term: `is(…, not(legendary))`.
- **`Status` stays a sort**, as `Color` does. Its values are `tapped`, `untapped`, `flipped`,
  `faceDown`, `faceUp`, `phasedIn` and `phasedOut`. A status printed as an adjective is spelled:
  "tapped", "untapped", "flipped", "face-down", "face-up", "phased-out". `phasedIn` has no
  spelling. Unhyphenated "face down" and "face up" are depictives: Oracle text prints them after
  a verb and its object ("exile it face down", "turn target creature face down", "put … onto the
  battlefield face down"), not as a modifier after a head, so no word for them stands in `post`,
  and the verbs that print them write them as their riders do today until
  `syl-verb-phrase-dependents` gives the depictive its place. Two in-scope cards print one after a
  copula, *Unable to Scream* ("as long as enchanted creature is face down") and *Showstopping
  Surprise* ("if it's face down"): the position-bound `Predicative` Template `isFaceDown` over
  the status value. The phasing wordings ("phases out") are verbs. `becomeStatus` and `entersAs`
  take a `Status`. The `PreModifier` Templates `tapped`, `untapped`, `faceDown` and `phasedOut`
  go.

## 5. What each declaration becomes

Every declaration at `PreModifier`, `PostModifier`, `Predicative` and `Predicate` in
`plugins/builtin/`, with the card whose parse decides it.

### One declaration at a kind

| Kind | Declarations | Decided by |
|---|---|---|
| `AdjectivePhrase` | `hasColor`, `hasSupertype`; `colorless`, `multicolored`, `historic`, `unblocked`, `transformed`; `other`; `otherThan`; `tiedFor` | "red" 496 before a head, 21 after a copula (*Fry*); "legendary" 334 / 74; "colorless" 539 / 15; "other" 1,567 before a head; "other than" 167 after a head; "tied" 34 after a copula (its three callers print it after "are") |
| `Nominal` | `token`, `permanent`, `outlaw` (each already a head); the type and subtype words through their coercion | "token" 350, "permanent" 355, "outlaw" 10 before a head |
| `VerbPhrase` | `attacking`, `blocking`, `blocked`, `chosen`, `equipped`, `foretold`, `plotted` and `was` at `PreModifier`; `attachedTo`, `named` and `was` at `PostModifier` | *Smite* "blocked", *Stone Haven Outfitter* "Equipped", *Bioplasm* "exiled"; "attach" passive 178 after a head; "name" passive 333 after a head; *Burn at the Stake* "tapped this way" |
| `PrepositionPhrase` | `in`, `inPile`; `hasKeyword` ("with $keyword"), `hasCounter`, `hasPower`, `hasManaValue`, `hasLesserManaValueThan`, `activationCostHas`, `manaCostHas`, `tapOrUntapInCost`, `withDifferentNames`, `withTheSameName`; `of`, `fromSource`, `ofTheLastChosen`, `ofChosen`, `ofYourChoice` | "in" 1,624 after a head, 313 after a copula (*Animate Dead*); *Aerial Volley* "with flying"; *Cursed Totem* "of creatures"; *Gaea's Revenge* "from nongreen sources"; *Adaptive Automaton* and *Crippling Fear* "of the chosen type" |
| `RelativeClause` | `youControl`, `youDontControl`, `youOwn`, `opponentsControl`, `anOpponentControls`, `controlledBy` ("$who control"), `ownedBy`, `actorControls`; `thatIs`, `couldEnchant`, `triggeredNotLeftStack`; `did` at `PostModifier` | *Academy Journeymage* "you control"; *Fry* "that's white or blue"; *Bruna, Light of Alabaster* "that could enchant it"; *Alabaster Leech* "you cast" |

The participle Templates at `VerbPhrase` are the verb phrases of
`docs/specs/voice-and-object-phrases.md` standing in `pre` and `post`:
`syl-passive-participle-and-trigger-events` replaces each fixed participle with its verb, and
`syl-lookback-operator-and-holds` replaces `did` and `was`. Until then each stays a Template at
the kind.

### Deleted as a duplicate

`hasColor`, `in`, `ofChosen`, `otherThan` and `transformed` at `Predicative`, and the
`Color → Predicative` coercion: the same phrase as the `pre` or `post` declaration, now reached by
membership (*Fry*, *Battlegate Mimic*, *Animate Dead*, *Crippling Fear*).

### Words of the union

- `PreModifier`: `or`, `andOr`, `coord`, `non`, `not`. `stacked` goes (§3).
- `PostModifier`: `or`, `andOr`, `coord`. `not` goes: its callers print other wordings, and each
  writes what it prints: a `without` preposition phrase ("without flying"), "with no …" as its own
  wording, a relative clause over the predicative `not` ("that aren't legendary"), or `not` inside
  the participle (*Escaped Shapeshifter*'s "not named …").
- `Predicative`: `and`, `or`, `andOr`, `both`, `coord`, `not`.
- Position-bound Templates: `withMostVotes` at `PostModifier` (*Council's Judgment*), and
  `hasTypeIgnoringTypeChanges` (sunburst's rule) and `isFaceDown` (§4) at `Predicative`.

### Identity wrappers deleted

`noun` at `Predicative` (`NounPhrase` is a member; *Archon of Coronation*'s "you're the monarch")
and `hasQuality` at `PreModifier` (a bridge from `Predicate`, 22 rule-body sites, which goes with
its consumers).

### Unspelled tests

A test with no wording of its own stands unspelled under the Templates that word it.

- **Worded at one kind**, it stands at that kind: `colorCount` and `chosenRef` (the Templates
  over them are adjectives and participles); `agree` (with-phrases), `legalAttachment` and
  `targeting` (relatives; *Gnarlback Rhino*'s "that targets this creature") and `choseExtreme`
  (*Menacing Ogre*'s "with the highest number").
- **Worded at several kinds**, it stands at `Predicate`: `hasPossessor` ("you control", a
  relative; "controlled by you", a participle), `inCombat` ("attacking you"; "that's blocking
  it"), `predicateDid`, `compare` (*Abrupt Decay*'s with-phrase; *Keeper of the Flame*'s "who has
  more life than you do"), `shares`, `superlative`, `hasDesignation`, `attachment`, `currentFace`
  and `related`, one declaration each in place of today's copy per position. A Template at a
  kind sort may have a body at `Predicate`; that is the one way a `Predicate` value reaches a
  kind, and no call site writes one bare. `Predicate` retires when the clause and voice tickets
  compose each wording from its verb and its other consumers (§6) have left.

### Wordings another ticket composes

| Today | Prints | Becomes | Ticket |
|---|---|---|---|
| `attachedTo`, `attachedBy`, `controlledBy`, `was`, `named` at `Predicative` | "are already attached to it" (*Benevolent Blessing*), "was cast from your hand" (*Approach of the Second Sun*) | the passive, `be(done(…))` | `syl-passive-participle-and-trigger-events` |
| `inCombat` at `Predicative` | "isn't being declared as an attacker" (*Verity Circle*) | a progressive over a passive | `syl-passive-participle-and-trigger-events` |
| `did` at `Predicative` and `Predicate`, `predicateDid` | "if you cast it" (*Arcane Proxy*) | `did(phrase, within)` under `holds` | `syl-lookback-operator-and-holds` |
| `hasCounter`, `hasKeyword`, `compare`, `shares` at `Predicative` | "has no ice counters on it" (*Dark Depths*), "didn't have defender" (*Bristlepack Sentry*), "has the same name as" (*Candles of Leng*) | a verb phrase headed by `have` | `syl-have-verb-phrases` (NEW) |
| `hasDesignation` at `Predicative` | "you're the monarch" (*Archon of Coronation*), "while saddled" (*Autarch Mammoth*), "has the city's blessing" (ascend) | a noun phrase, an adjective phrase and a `have` phrase, one wording each | `syl-have-verb-phrases` (NEW) |
| `named` at `Predicative` (2 canon callers) | "has the chosen name" | a `have` phrase | `syl-have-verb-phrases` (NEW) |
| `choseExtreme` at `Predicative` | "If you are one of those players" (*Menacing Ogre*) | a noun phrase after the copula | `syl-canon-wording-defects` |
| `copied` at `PreModifier` | "the copy" (*Rings of Brighthearth*) | a noun phrase | `syl-canon-wording-defects` |
| `notChosen` at `PostModifier` | "not chosen this way" (*Celestial Judgment*), "that weren't chosen this way" (*Disciple of Caelus Nin*) | `not` inside the participle; a relative over the passive | `syl-passive-participle-and-trigger-events` |
| `outsideTheGame` at `PostModifier` and `Predicative` | "from outside the game" (19 of 22 units) | a place, the complement of "from" | `syl-zone-words` |
| `Status → Predicative`, `hasStatus` at `PreModifier` and `Predicative` | "tapped" (*Helm of Possession*), "face-down" | the `Status → AdjectivePhrase` coercion (§4) | the membership ticket |

### `Predicate`

Its 36 declarations retire with their consumers:

- **Keyword qualities** (`hasType`, `hasSubtype`, `hasSupertype`, `hasColor`, `isCard`, `isSpell`,
  `isPermanent`, `isPlayer`, `isOpponent`, `isManaAbility`, `abilityHead`; `eachKind`,
  `objectsOfPlayer`; and the `PostModifier` copies `in`, `named`, `hasKeyword`, `youControl`,
  `anOpponentControls`, `controlledBy`, `ownedBy`, `ofChosen`, `ofYourChoice`, `ofTheLastChosen`,
  `did`, `isTransformed`): the 13 keywords whose parameter is `quality: Predicate` take a
  `Quality`, and each call writes the nominal, noun phrase or adjective phrase its card prints
  ("Enchant creature card in a graveyard", *Animate Dead*; "protection from each color among
  permanents you control", *Pledge of Loyalty*). `syl-keyword-quality-union` (NEW).
- **Coordination and negation** (`and`, `or`, `coord`, `not`, `non`): the union's or the
  nominal's own, once no quality stands at `Predicate`.
- **Rule helpers and the parked tests** (`compare`, `currentFace`, `hasPossessor`, `predicateDid`,
  `partyRoles`, `isOpponentOf`): stay at `Predicate` until the wordings over them are composed.
- **Spend purposes** asked for at `Predicate` by `syl-tocast-noun-phrase`,
  `syl-turn-face-up-spend-cost` and `syl-cast-permission-spell-anaphor` are written at a kind
  sort by those tickets.

## 6. Tickets

Built by, in order:

1. **`syl-union-membership`** (NEW): the `union` declaration, step 5, Reaches and the load rules
   of §3 in `cargo xtask dsl check`; the three kind sorts; the four unions; §4's coercions; every
   "one declaration at a kind", duplicate, union word, wrapper and unspelled-test row of §5, with
   canon rewritten where a name changes; `not` at `PostModifier` removed. It declares
   `RelativeClause` and moves today's fixed relatives onto it.
2. **`syl-keyword-quality-union`** (NEW): the 13 keyword parameters at `Predicate` become
   `Quality`.
3. **`syl-have-verb-phrases`** (NEW): the "has …" wordings.

Changed by this spec: `syl-zone-words` (a location phrase is a `PrepositionPhrase`),
`syl-damage-and-life-noun-phrases` (its modifiers are declared at kinds),
`syl-passive-participle-and-trigger-events`, `syl-lookback-operator-and-holds` and
`syl-clause-and-imperative` (the participle, `did` and `was`, and a relative composed from a
clause with a gap, each at its kind), `syl-become-replaces-types` and
`syl-token-records-to-templates` (they consume `Predicative`, a union), and
`syl-name-originally-printed-in`, `syl-permanent-type-read`, `syl-daybound-reconfigure-bodies` and
`syl-clone-any-on-the-battlefield` (each asks for a new word at `Predicate`, which is declared at
its kind instead). `syl-verb-phrase-dependents` builds a depictive ("enters tapped") from the same
kinds; `MoveRider` stays until it does.

## 7. Not decided here

- Declaring a coordinator once for every sort it coordinates.
- A typed Rust representation of the surface term, in which a union is an enum over its members.

## 8. What the design docs say once this is built

| Document | Today | Then |
|---|---|---|
| `syl.md` §2 | "There are nine" coercions, four into `PreModifier` and two into `Predicative` | §4's six; a `union` declaration beside `coercion` |
| `syl.md` §3 | resolution steps 1 to 4; Reaches(`T`) is `T` and the from-sorts of coercions into it; `tapped` at `Status` and at `PreModifier` is no collision | §3 here |
| `surface.md` §1 | "`Nominal` and its modifier sorts" | the kind sorts of §1; a union is a sort that names a place kinds stand, not a kind |
| `spelling.md` §2 | `stacked` for an arm of two premodifiers; `thatIs(predicative)` at `PostModifier`; a color at `Predicative` "through its coercion" | a nominal arm (§3); `thatIs` at `RelativeClause`; membership |
| `spelling.md` §7 | "A sort is not one Category: a `PreModifier` is a noun in 'creature card' and an adjective in 'tapped creature'" | a kind sort is one Category and a union is a function; the skeleton rule of §3 |
| `semantics.md` §2.4 | "at `PreModifier` it tests the characteristic alone"; "`Predicative`'s noun-phrase form `noun(phrase)`" | the type word's reading is positional (§4); a noun phrase is a member of `Predicative` |

Glossary entries (`docs/contexts/game-model/CONTEXT.md`):

- **Coercion**: its examples become a type as a Nominal, a color, supertype or status as an
  AdjectivePhrase, an integer as an Amount.
- **PreModifier**, **PostModifier**, **Predicative**: each is a union, defined by its members and
  the place it types.
- **Union** (NEW): a sort whose values are the values of its member sorts.
