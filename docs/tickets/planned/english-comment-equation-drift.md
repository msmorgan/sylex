---
needs: [english-category-restriction, english-features-as-tags]
---
# Comments that claim more than, or other than, their equations enforce

## Why

Several comments in `crates/english/grammar/english.fug` state restrictions their equations do not
implement, or state a corpus or style observation as a fact of English (`docs/design/english.md`
§6):

- `FusedPostmodifier`: "Only a marked relative follows a fused head", while
  `FusedParticipialPostmodifier`, right after it, adds a participial postmodifier to the same
  Category.
- `valence VerbPhrase`'s `shifted_object: [object.Determination := Overt]`: "A shifted heavy
  Object is determined", stating a corpus gate as a general heavy-shift condition.
- `AsyndeticAttributes`, `AttributeSequence`, `FinalAttribute`: grounded in the style guide alone.
- `FloatedAll`: *all* floats "after a personal pronoun
  Subject only", whereas CGEL p. 413 lists *all*, *both* and *each* together with full Noun Phrase
  Subjects.
- `FusedDegreeDeterminer`: "Count only: never *that much*", a register gate behind
  `$.Countability = Count`.
- `NounAttribute` and `NominalPremodifier`: "A noun premodifier sits inside determinative and
  adjectival ones, never outside them", while `NominalPremodifier` checks only
  `head.DeterminerNeed != Required`.
- `FiniteComplementClause` (`{ unlinked: [head] }`): states "a sole connective never links
  subordinate clauses" without naming the Oracle `, then` `LinkedClause` scope gate or the 3 real
  cards whose spurious `, then` capture it prevents (*Ensnared by the Mara*, *Transforming
  Flourish*, *Kraul Harpooner*).

## Done when

- Each comment above states what its equations actually enforce, cites CGEL accurately, and where a
  restriction is an admission gate (`docs/design/english.md` §6), names a real card and the spurious
  structure prevented (or tightens the equations where intended, with no Reading lost on the
  supported corpus).
- `cargo xtask english strata --check`, `cargo test -p sylex_english` and
  `cargo clippy -p sylex_english --tests -- -D warnings` pass.

## Pointers

- `crates/english/grammar/english.fug`: the declarations above, `RelativeTemporalLocation`,
  `FiniteComplementClause`.
- `docs/design/english.md` §6.
- Local evidence (gitignored): `docs/evidence/english-fug-hax-check/fable.md` (F5, F10),
  `docs/evidence/english-fug-hax-check/gemini.md` (E1, E3).
