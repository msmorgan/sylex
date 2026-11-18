---
needs: []
---
# A temporal Adjunct *next turn* without a determiner

## Why

*Peace Talks* prints *This turn and next turn, creatures can't attack, ...*: a coordination of two
temporal location Noun Phrases, *this turn* and *next turn*. CGEL lets *next* stand as a period
noun's only dependent in a temporal location Noun Phrase (*next week*; Ch. 8 §6.3, p. 698), but
the grammar builds no Noun Phrase *next turn* (the adjective *next* needs a determiner, and
there is no determinative *next*), so the unit's one analysis of its Adjunct is wrong: *this
[[turn and next] turn]*, an attributive coordination under *this*.

## Done when

- *Peace Talks* line 1 has a Reading whose preposed Adjunct is the coordination of *this turn*
  and *next turn*, and none in which *this* determines *next turn* or *turn and next* modifies
  *turn*.
- *during its controller's next untap step* and the other genitive-determined *next* phrases keep
  their `GenitiveDetermined` Readings and gain no bare *next* Adjunct.
- The census shows no verdict loss, every Corpus Unit whose Readings change is checked, and
  `cargo test -p sylex_english` and `cargo clippy -p sylex_english --tests -- -D warnings` pass.

## Pointers

- `crates/english/grammar/english.fug`: `TemporalLocation`, `PreposedTemporalAdjunct`.
- `crates/vocabulary/lexicon/english/adjectives.ron` (*next*),
  `crates/vocabulary/lexicon/english/closed_class.ron` (`determinatives`).
