---
needs: []
---
# Coordination gates that may refuse grammatical coordinations

## Why

Four coordination constructs appear to refuse grammatical text or to cover only part of a
general construction:

- **Overt then bare mass.** Four `coordination NounPhrase` schemas exclude
  `!(first.Determination = Overt & last.Determination = Bare & last.Countability = Mass)`, so
  that in *its power and toughness* the determiner scopes over both nouns. Stated as a fact of
  English, it also refuses a determined Noun Phrase coordinated with a bare mass one where the
  mass noun takes no determiner (*from your graveyard or exile*). The *power and toughness* case
  may come from *power* and *toughness* being listed as both count and mass in the lexicon,
  against the lexicon's own comment that a bare *power* or *toughness* is never a Noun Phrase.
  Both scopes of *its power and toughness* are grammatical English, attachments to different
  heads that ADR §4 keeps packed unless a gate is measured.
- **Noun premodifier conjuncts.** `adjunct NounPremodifier { host: NounPremodifierConjunct }`
  requires `modifier.Polarity := Negative`, refusing a positive noun premodifier inside a
  conjunct (*instant or [artifact creature] spell*). Both it and `adjunct AdjectivePhrase {
  host: NounPremodifierConjunct }` are `side: Before`, which the engine appears to exclude on a
  coordination's first conjunct, so *[legendary artifact] or enchantment card* may have only
  the reading where both are legendary.
- **Bare relatives.** `coordination RelativeClause { coordinators: Coordinator }` requires
  `last.ClauseMarking = Relative`, while the `{ coordinators: But }` schema has no equation, so
  bare relatives coordinate with *but* (*a permanent you own but an opponent controls*) and
  apparently not with *and* or *or*. `SubjectInitialCoordination` is used only by
  `CoordinatedAdverbialRelative`, not by `BareRelative`.
- **Contrast.** `ContrastConjunct`, `ContrastCoordination`, `ContrastPrepositionConjunct` and
  `ContrastPrepositionCoordination`, with their Productions and two coordination schemas, write
  *X but not Y* (CGEL Ch. 15 §2.6, p. 1313) twice, for `ComplementNounPhrase` and
  `PrepositionPhrase` only.

## Open questions

- For overt-then-bare-mass: keep the gate with its ablation stated, or delete it and keep both
  scopes, fixing the count/mass listing of *power* and *toughness* if that is the cause?
- Should contrastive *but not* become a coordination option derived once, or stay as two
  Category pairs?

## Done when

- Each "appears" and "may" above is confirmed or refuted by parse (*from your graveyard or
  exile*, *instant or artifact creature spell*, *legendary artifact or enchantment card*, *a
  permanent you own and an opponent controls*), and the engine's first-conjunct rule for
  `side: Before` adjuncts is confirmed in `crates/syntax/src/step.rs`. A refuted item needs no
  change.
- The overt-then-bare-mass exclusion is ablated and the result recorded.
- Each confirmed defect is fixed or ticketed, the bare relative coordinators behave alike, and
  each gate kept states its ablation.
- The census shows no Reading lost on the supported corpus.

## Not in scope

- `ObjectCoordination`: `english-object-coordination-exclusions`.

## Pointers

- `crates/english/grammar/english.fug`: `coordination NounPhrase`,
  `category NounPremodifierConjunct`, `adjunct NounPremodifier`, `adjunct AdjectivePhrase`
  (host `NounPremodifierConjunct`), `coordination NounPremodifier`, `NounConjunct`,
  `coordination RelativeClause`, `coordination SubjectInitialCoordination`, `BareRelative`,
  `CoordinatedAdverbialRelative`, the `Contrast*` categories and coordinations,
  `AffirmativeConjunct`, `NegatedConjunct`, `ContrastedComplement`,
  `AffirmativePrepositionConjunct`, `NegatedPrepositionConjunct`,
  `ContrastedPrepositionPhrase`.
- `crates/vocabulary/lexicon/english/nouns.ron` (*power*, *toughness*, *exile*).
- Local evidence (gitignored): `docs/evidence/english-fug-hax-check/gemini.md` (D2, D3, E2,
  E4), `docs/evidence/english-fug-hax-check/fable.md` (F10).
