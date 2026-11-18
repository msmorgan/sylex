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
- `AsyndeticAttributes`, `AttributeSequence`, `FinalAttribute`: the `Polarity := Negative` gate
  names its card (Farseek) but is stated as a fact of Oracle style (OSG §6), and the comma
  construction it restricts is given no CGEL ground.
- `FloatedAll` and the `FloatingQuantifier` class: *all* floats "after a personal pronoun
  Subject only", whereas CGEL p. 413 lists *all*, *both* and *each* together with full Noun Phrase
  Subjects.
- `FusedDegreeDeterminer`: "Count only: never *that much*", a register gate behind
  `$.Countability = Count`.
- `NounAttribute`: "A noun premodifier sits inside determinative and adjectival ones, never
  outside them", while `NominalPremodifier` orders a noun only by `head.DeterminerNeed !=
  Required` (its other two gates concern *counter* and *expansion*), so *a Goblin red creature
  token* parses with *Goblin* outside *red*.
- `FiniteComplementClause` (`{ unlinked: [head] }`): states "a sole connective never links
  subordinate clauses" as a fact of English, without naming the scope gate it is or a real card
  whose spurious `, then` capture it prevents (removing it adds a Reading on 14 units, among them
  *Demonlord Belzenlok*, *Ensnared by the Mara* and *Transforming Flourish*).

## Done when

- Each comment above states what its equations actually enforce, cites CGEL accurately, and where a
  restriction is an admission gate (`docs/design/english.md` §6), names a real card and the spurious
  structure prevented (or tightens the equations where intended, with no Reading lost on the
  supported corpus).
- `cargo xtask english strata --check`, `cargo test -p sylex_english` and
  `cargo clippy -p sylex_english --tests -- -D warnings` pass.

## Pointers

- `crates/english/grammar/english.fug`: the declarations above.
- `docs/design/english.md` §6.
- Local evidence (gitignored): `docs/evidence/english-fug-hax-check/fable.md` (F5, F10).
