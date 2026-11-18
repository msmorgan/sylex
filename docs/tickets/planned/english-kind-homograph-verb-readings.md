---
needs: [english-counter-kinds-read-from-text]
---

# A Kind Name's homograph as a verb rides on structures no card prints

## Why

A Kind Name holds no license (`docs/design/lexicon.md` §5), so where the word before *counter* is
also a verb the English may read it as one. Five units read so beside their kind:

- *Deadly Designs* line 2 (4 Derivations, 3 with *plot* a verb): `[there are five] or [more plot
  counters on this enchantment]`, a clause coordination that splits *five or more*, *more* the
  Subject of *plot*.
- *Cryptex* line 2 (14, 11 with *unlock* a verb): the same split, and causative *has [five or
  more] [unlock counters on it]*.
- *Fylgja* line 2 and *Ursine Fylgja* line 1 (5 each, 3 with *healing* a verb): *enters with
  four* and a gerund-participial Adjunct *healing counters on it*.
- *Ajani Unyielding* line 3 (11, 6 more than its 5): *[five loyalty] counters on each other
  planeswalker*, *counters* a finite verb in a coordinated relative clause.

Each is a structure of the grammar's beyond the Kind Name. Where no card prints the structure, an
admission gate grounded in card text removes the Reading (`docs/design/english.md` §6).

## Done when

- For each structure above, the landing shows with forest evidence whether a card prints it
  elsewhere; each that no card prints is refused by a gate whose comment names the card it
  protects, and each that a card prints is named with that card.
- The census shows no verdict loss, and `cargo xtask english strata --check`, `cargo test` and
  `cargo clippy --tests -- -D warnings` pass for every changed crate.

## Pointers

- `crates/english/grammar/english.fug` (clause coordination, `Premodifier`, causative *have*,
  gerund-participial Adjuncts, cardinal determiners of a mass noun).
- `target/english/forests.sqlite` of the landing, or `data/derived/english-forests.sqlite` after it.
