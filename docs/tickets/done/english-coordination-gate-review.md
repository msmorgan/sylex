---
needs: []
---
# Admit bare-mass NP conjuncts after overt NPs and align bare-relative coordinators

## Why

Two coordination gates in `crates/english/grammar/english.fug` refuse grammatical coordinations:

- **Overt then bare mass.** Four `coordination NounPhrase` schemas exclude
  `!(first.Determination = Overt & last.Determination = Bare & last.Countability = Mass)` so *its
  power and toughness* builds only the shared-determiner Nominal coordination. That exclusion also
  refuses `"Exile target card from your graveyard or exile."` (`0` Derivations today), while
  `power` and `toughness` genuinely use `BareMass` (*with power 3*, *with total power 8 or greater*,
  making `nouns.ron`'s comment that bare *toughness* is never a Noun Phrase stale), and both scopes
  of *its power and toughness* are grammatical English (`docs/design/english.md` §4, §8).
- **Bare relatives.** `coordination RelativeClause { coordinators: Coordinator }` requires
  `last.ClauseMarking = Relative`, while `{ coordinators: But }` has no equation, so `"Destroy
  target permanent you own but an opponent controls."` parses while `"Destroy target permanent you
  own and/or an opponent controls."` has `0` Derivations, even though `RelativeClause` already
  requires `Gap = NP` (CGEL, Ch. 12, §3.5.2, p. 1059).

## Design

- Delete `!(first.Determination = Overt & last.Determination = Bare & last.Countability = Mass)`
  from the four `coordination NounPhrase` schemas and update the stale comment on `power` and
  `toughness` in `crates/vocabulary/lexicon/english/nouns.ron`.
- Drop `last.ClauseMarking = Relative` on `coordination RelativeClause` so bare relative clauses
  coordinate with `and`, `or` and `but` alike.

## Done when

- `"Exile target card from your graveyard or exile."` and `"Destroy target permanent you own and/or
  an opponent controls."` each parse, `nouns.ron`'s `power`/`toughness` comment matches their
  `[Count, Mass]` entries, and bare relative coordinators behave alike.
- Every corpus unit whose Readings change is checked, the census shows no Reading lost on the
  supported corpus, and `cargo xtask english strata --check`, `cargo test -p sylex_english` and
  `cargo clippy -p sylex_english --tests -- -D warnings` pass.

## Not in scope

- Noun premodifier conjuncts and `Fronting::Wide`: `english-premodifier-over-coordinated-heads`.
- Deriving contrastive `but not` in `coordination`: `english-category-restriction`.

## Pointers

- `crates/english/grammar/english.fug`: `coordination NounPhrase`, `coordination RelativeClause`,
  `BareRelative`.
- `crates/vocabulary/lexicon/english/nouns.ron` (*power*, *toughness*, *exile*).
