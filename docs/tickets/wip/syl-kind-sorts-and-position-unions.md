---
needs: []
---

# Spec: modifiers are sorted by kind of phrase, and positions are unions

## Why

A sort names a kind of phrase (`docs/design/surface.md` §1). `PreModifier`, `PostModifier` and
`Predicative` name a position instead: `spelling.md` §7 says a `PreModifier` is a noun in
"creature card" and an adjective in "tapped creature", and a `PostModifier` a preposition phrase
in "with flying" and a relative clause in "you control". The position is already given by which
list of `nominal` a phrase stands in, or by the copula, so the sort records it twice, and the
bundle declares one phrase once per position: 24 names stand at both `PostModifier` and
`Predicative` (`in`, `named`, `otherThan`, …), `hasColor` at three sorts, `not` at four, and
`thatIs` and the `Color → Predicative` coercion exist only to carry a phrase from one position to
another. The copies drift (`attachment`, `inCombat` and `hasDesignation` have a different
signature per sort) and leave gaps: "isn't legendary" (*Yenna, Redtooth Regent*) has no term, and
*Balance*'s "lands controlled by the player who controls the fewest" cannot be written.

`crates/english/grammar/english.fug` already keeps the two apart: `Predicative`, `Postmodifier`
and `Attributes` are `{ function }` Categories filled by unary productions over the kinds of
phrase. `docs/specs/voice-and-object-phrases.md` already puts a `VerbPhrase` in a nominal's
`post`, and nothing says how it gets there.

## Design

- The kinds of phrase are sorts: `AdjectivePhrase` ("red", "legendary", "tapped", "other than
  X"), `PrepositionPhrase` ("in your graveyard", "with flying", "from nongreen sources", "face
  down"), `RelativeClause` ("you control", "that's white or blue"), and the existing
  `VerbPhrase` (as a participle: "exiled", "attacking", "controlled by you"), `NounPhrase` and
  `Nominal`. Each phrase is declared once, at its kind.
- `PreModifier`, `PostModifier` and `Predicative` stay as sorts and are **unions** of kinds: a
  relation between sorts beside coercion, with free membership and no body. `PreModifier` is an
  adjective phrase, a nominal or a verb phrase; `PostModifier` a preposition phrase, a relative
  clause, a verb phrase or an adjective phrase; `Predicative` an adjective phrase, a preposition
  phrase or a noun phrase. A value takes at most one coercion and at most one membership step, so
  a coercion stays one hop (`syl.md` §3).
- Membership is by kind. It admits a wording English does not print in that position ("is
  face-down"); no finer sort refuses it.
- A nominal is a member of `PreModifier`, so a type or subtype word is one word wherever it
  stands, "creature" in "creature card" and "Goblin" in "Goblin card" alike, and `token` and
  `permanent` are one declaration as head and as modifier. The `CardType → PreModifier` and
  `Subtype → PreModifier` coercions and `stacked` go. That a description with a type word and no
  "card", "spell" or "source" means a permanent on the battlefield [CR#109.2] is read by semantics
  from where the word stands, not written by the term.
- A union declares only words from the union to itself (coordination, negation, a prefix), and
  its own declaration is the one a name resolves to there.
- An unspelled test whose wordings are of several kinds (`hasPossessor`: "you control", a relative
  clause, and "controlled by you", a participle) stands at `Predicate`, which every kind admits as
  a Template's body, until the clause and voice tickets compose each wording from its verb.
  `Predicate` retires when the last of them and its other consumers leave.
- A depictive ("enters tapped", "put onto the battlefield face down") is built from the same
  kinds, so no word is declared again for standing after a verb; `MoveRider` stays as it is until
  `syl-verb-phrase-dependents` replaces it.
- A keyword's quality parameter ("protection from red", "enchant creature", "from everything") is
  typed by one more union, `Quality`: an adjective phrase, a nominal or a noun phrase. Keywords
  declare their parameters as they do now; a later ticket retypes the ones at `Predicate`.
- `post` holds every dependent printed after the head in printed order, a complement the head
  selects ("half of this split card") and a modifier alike; the term does not mark which.
- `Status` stays a sort, as `Color` does, and one coercion `Status → AdjectivePhrase` carries a
  spelled status into every position. "Tapped", "untapped", "flipped", "face-down" and "face-up"
  are spelled; `phasedIn` and `phasedOut` are not. Unhyphenated "face down" and "face up" and the
  phasing wordings are Templates in their idioms over the status value. `becomeStatus` and
  `entersAs` take a `Status`; `hasStatus` and the `Status → Predicative` coercion go.
- `not` at `PostModifier` goes. Each caller writes what it prints: a `without` preposition phrase,
  "with no …" as its own wording, or a relative clause over the predicative `not`. `non-` stays at
  `PreModifier` and `not` at `Predicative`.
- The ticket that builds union membership declares `RelativeClause` and moves today's fixed
  relatives onto it; `syl-clause-and-imperative` composes a relative from a clause with a gap.
- A union's membership declaration states the form a verb phrase takes there
  (`VerbPhrase@GerundParticiple` in `PreModifier` and `PostModifier`), with the form labels of
  `spelling.md` §7: an active verb phrase prints its gerund-participle ("attacking"), and a `done`
  phrase its past participle with no "be".

## Done when

- `docs/specs/kind-sorts-and-position-unions.md` states the design above and settles what it
  leaves open: the membership declaration's syntax; where membership falls in a word's resolution
  order (`syl.md` §3) and how Reaches and the ambiguous-skeleton load rule (`spelling.md` §7)
  count members; each union's members, checked against the pool's parses; and, for every
  declaration at `PreModifier`, `PostModifier`, `Predicative` and `Predicate`, what it becomes
  (one declaration at a kind, a deletion, a word of the union, a test parked at `Predicate`, or a
  wording another ticket composes), with the card whose parse decides it.
- The spec says what each of these becomes and which ticket owns it: the passive and progressive
  wordings at `Predicative` (`attachedTo`, `controlledBy`, `was`, `named`, `inCombat`); the "has
  …" clauses (`hasCounter`, `hasKeyword`, `compare`, `shares`), which no planned ticket owns;
  `Status`, whose words are an adjective ("tapped"), a preposition phrase ("face down") and a
  third word ("face-down"); `did` and `was`; and the keyword qualities and spend purposes at
  `Predicate`.
- `syl.md` §2 and §3, `surface.md` §1, `spelling.md` §2 and §7 and `semantics.md` §2.4 agree with
  the spec, and the glossary entries **Coercion**, **PreModifier**, **PostModifier** and
  **Predicative** in `docs/contexts/` do too.
- The tickets that build the spec exist in `planned/`, each naming it in `specs:`, and the planned
  tickets it changes say so: `syl-zone-words`, `syl-passive-participle-and-trigger-events`,
  `syl-lookback-operator-and-holds`, `syl-clause-and-imperative`, and each ticket that asks for a
  new word at `Predicate` or consumes `Predicative`.
- The owner has read the spec before any ticket that builds it is claimed.

## Not in scope

- Declaring a coordinator once for every sort it coordinates.
- The verb phrase's dependent list and `MoveRider`'s removal: `syl-verb-phrase-dependents`.
- A typed Rust representation of the surface term.

## Pointers

- Evidence: `docs/evidence/syl-kind-sorts/research.md` (the findings, pool counts and experiments)
  and `declaration-table.md` (every declaration classified).
- `plugins/builtin/{primitives,templates}/{preModifier,postModifier,predicative,predicate}/`,
  `plugins/builtin/coercions/`;
  `crates/xtask/src/dsl_check/{coercion,nominal,schema,sort,spelling}.rs`
- `crates/english/grammar/english.fug` (the `{ function }` Categories and their productions);
  `crates/english/README.md` for reading a card's parse
- `docs/design/{syl,surface,spelling,semantics,architecture}.md`;
  `docs/specs/voice-and-object-phrases.md` §2, §8, §10; `docs/specs/README.md`
- `docs/tickets/done/syl-nominal-descriptions.md`,
  `docs/tickets/done/syl-nominal-modifier-sorts-and-checks.md`
