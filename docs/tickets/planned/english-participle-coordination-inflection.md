---
needs: []
---
# A Participle coordination has no Inflection, so a gate on it tests nothing

## Why

`coordination Participle` lists `Inflection` as unshared, so every Participle coordination
leaves it unvalued, a coordination of two gerund-participles included (*attacking or
blocking*). `ParticipialAttribute`, "a gerund-participle as an unlike conjunct of an
adjective", tests `filler.Inflection = GerundParticiple`, which an unvalued feature passes. It
therefore takes a coordination of past participles as a gerund-participial Attribute:
*Destroy each tapped or exiled or destroyed creature.* reads through
`ParticipialAttribute` over *exiled or destroyed*. Writing `:=` instead refuses that and also
the correct *tapped or attacking or blocking creature*, whose coordination has no Inflection
either. No supported-corpus unit changes under either form of the equation.

## Done when

- A Participle coordination whose conjuncts share an Inflection carries it, and one that
  mixes a gerund-participle with a past participle leaves it unvalued.
- `ParticipialAttribute` requires the gerund-participle with `:=`, and its comment says what
  the equation does.
- *Destroy each tapped or attacking or blocking creature.* keeps its Reading, and a
  coordination of past participles is no gerund-participial Attribute; an engine test or a
  judgment on a real card's text pins each, as `crates/english/judgments/README.md` allows.
- Every other declaration that reads a Participle's `Inflection` is checked for the same
  unvalued case.
- The census shows no verdict change on the supported corpus, and every unit whose Readings
  change is checked.

## Not in scope

- Whether a past participle is an Attribute conjunct at all (*tapped or exiled creature* has
  no Reading).

## Pointers

- `crates/english/grammar/english.fug`: `coordination Participle`, `ParticipialAttribute`,
  `category Participle`; the comparative-last `coordination AdjectivePhrase` schema, which
  values a feature of a coordination from its last coordinate.
- `crates/syntax/src/grammar.rs` (coordination `unshared`, `resolve`),
  `crates/syntax/tests/coordination_resolution.rs`.
