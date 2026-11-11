---
needs: []
---
# Productions for one or two cards whose comments state a general rule

## Why

These declarations fit the strings of one or two cards (or one phrase) while their comments
state a rule of English:

- `QuotedCost`, `QuotedCostPremodifier`: a quoted imperative is a cost noun only after an
  attributive adjective (*an additional "Sacrifice a ..."*; Brutal Suppression, Drought);
  nothing about quoted costs makes the premodifier obligatory, and a quoted cost under a
  determiner alone has no Reading.
- `ParticipialVerblessClause`: a passive Verb Phrase plus exactly one Preposition Phrase
  Adjunct, to tell *if tapped for mana* from *if tapped*; a reduced passive whose dependent is
  inside the Verb Phrase (*if exiled with ...*, *if chosen this way*) may have no Reading. No
  card is named.
- `LocatedExistentialBe`, `LocatedProximateExistentialBe`: the preposition list `In, On, Among,
  Under, Outside, After` restates *be*'s prepositional frame and adds `After`, which *be* does
  not select, for *an additional combat phase after this phase*; proximity agreement (CGEL Ch.
  4 §3.2.2, p. 242 n. 21) exists only in the located variant, not in `ExistentialBe`. No card
  is named.
- `PostposedBareRelative`, `PostposedParticipialModifier`: `adjunct.ComplementKind = AdjP`
  stated as "a bare relative postposes only past an adjectival Complement"; the Preposition
  Phrase with an adjective Complement is *at random* (also *in common*). No card is named.
- `FloatedObjectBoth`: Dream Cache (*both on top of your library or both on the bottom of your
  library*), where the second *both* cannot be floated off the Object.
- `feature Cardinality`, `class SumAdjective`, `SumQualifiedNounPhrase` and `coordination
  DeterminerPhrase`'s `resolve: [Cardinality -> Cardinal]`: one card (*ten or more creature
  cards total*, Avatar of Woe), with *total* the only sum adjective; a feature every
  determinative values, refusing bare and definite Noun Phrases no card prints.
- `PreposedFrequency`: `adverb.AdjunctPosition = Multiplying` picks out *twice*, though the
  `AdjunctPosition` comment says `Multiplying` is a multiplier that never goes with a temporal
  Noun Phrase; no card with a fronted *Twice this turn,* is known.

## Open questions

- For a Production with one supporting card: name the card in its comment and keep it, or
  handle the card another way (an Editorial Correction, a general rule, or none)?

## Done when

- For each declaration, the card(s) behind it are confirmed in the supported corpus, with the
  count recorded; a declaration with no card is deleted or its comment names what it serves.
- The behaviors stated as "may" above are confirmed or refuted by parse.
- Each declaration's comment states what its equations do and names its card(s), or the
  declaration is replaced as the owner rules.
- `LocatedExistentialBe`'s preposition list agrees with *be*'s frame or the comment says why
  not.
- The census shows no Reading lost on the supported corpus.

## Not in scope

- Citation fixes on these declarations: `english-cgel-citation-corrections`.
- `SpecifyingClause` (`english-specifying-clause`), `SynesisDistributivePartitive`
  (`english-synesis-each-partitive`), `InterleavedManner` (`english-interleaved-manner-gate`),
  the *tied for* Productions (`english-tied-for-superlatives`), `MeasureObjectConjunct`
  (`english-object-coordination-exclusions`), *outside the game*
  (`english-outside-the-game-frame`).

## Pointers

- `crates/english/grammar/english.fug`: the declarations above, `ExistentialBe`,
  `PostposedRelative`, `feature AdjunctPosition`.
- `crates/vocabulary/lexicon/english/closed_class.ron` (*be*'s frames),
  `crates/vocabulary/lexicon/english/adjectives.ron` (*total*'s `follows_sum`).
- Local evidence (gitignored): `docs/evidence/english-fug-hax-check/fable.md` (F9, F12),
  `docs/evidence/english-fug-hax-check/gemini.md` (C3, C5, C6, C7).
