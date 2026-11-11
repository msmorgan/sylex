---
needs: []
---
# Comments that claim more than, or other than, their equations enforce

## Why

These comments state a restriction the equations do not implement, or state a corpus or style
observation as a fact of English, which ADR §6 says is not by itself a gate's justification:

- `category RelativeTemporalNominal`: "a *when*-less adverbial relative, then at most a
  temporal bound, and no other postmodifier". `RelativeTime` takes `Nominal` (not
  `BareNominal`) and a `RelativeClause` with no `ClauseMarking` requirement, and
  `RelativeTimePostmodifier` is left-recursive, so it appears to admit marked relatives, other
  postmodifiers and any number of bounds.
- `FusedPostmodifier`: "Only a marked relative follows a fused head", while
  `FusedParticipialPostmodifier`, right after it, adds a participial postmodifier to the same
  Category.
- `valence VerbPhrase`'s `shifted_object: [object.Determination := Overt]`: "A shifted heavy
  Object is determined". The condition CGEL gives for heavy shift is weight; determination is a
  corpus observation.
- `TemporalLocation`: "never an article, genitive or postmodifier (CGEL Ch. 8 §6.3, p. 698)".
  The equation also excludes `Numerical` and every `Dependent` determiner, while the same page
  admits *one day* and *the following year*.
- `AsyndeticAttributes`, `AttributeSequence`, `FinalAttribute` (`category AttributeChain`):
  every attribute of an asyndetic chain is negative, grounded in the style guide alone.
- `FloatedAll` (and its twin `RelativeFloatedAll`): *all* floats "after a personal pronoun
  Subject only". CGEL p. 413 lists *all*, *both* and *each* together as floating, with a full
  Noun Phrase Subject example.
- `FusedDegreeDeterminer`: "Count only: never *that much*", a register claim behind
  `$.Countability = Count`.
- `NounAttribute` and `NominalPremodifier`: "A noun premodifier sits inside determinative and
  adjectival ones, never outside them". `NominalPremodifier`'s equation,
  `head.Determination != Overt`, only stops a noun before a nominal that demands a determiner;
  a noun before an ordinary adjective is admitted.

## Done when

- Each claim above is checked against the equations, and where it rests on CGEL, against the
  cited page; a claim found accurate needs no change.
- Each comment states what its equations enforce, and where a restriction is a gate, the
  ablation that justifies it; or the equations are changed to enforce what the comment states,
  with the census showing no Reading lost on the supported corpus.

## Not in scope

- The overt-then-bare-mass Noun Phrase coordination gate: `english-coordination-gate-review`.
- Where gate ablation records live: `english-gate-ablation-records`.

## Pointers

- `crates/english/grammar/english.fug`: the declarations above, `RelativeTemporalLocation`.
- `docs/decisions/english.md` §6.
- Local evidence (gitignored): `docs/evidence/english-fug-hax-check/fable.md` (F10),
  `docs/evidence/english-fug-hax-check/gemini.md` (E3).
