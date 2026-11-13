---
needs: []
---

# Which bare infinitival Complements give their head time

## Why

`dependents TimeGiving` lists `BareInf`, so every bare infinitival Complement gives its time to
the phrase it is in and may not differ in time from its head. That is right for a modal, whose
Complement's situation is simultaneous with the modal's time or later (CGEL, Ch. 3, §7,
p. 160), and for supportive *do*, which is not temporally distinct from its Complement (Ch. 14,
§4.2.2, p. 1217). The same Category is also the Complement of causative *have* (*you may have
target player reveal their hand*) and of *rather than* and *plus*, for which no passage of CGEL
has been found that makes the two one time.

A related gap: a `dependents` equation relates one dependent to the `head`, so two time-giving
dependents of one Production that differ in time are both admitted under a head that carries no
time.

## Open questions

- Whether the Complement of causative *have*, *rather than* and *plus* shares its head's time.

## Done when

- Each head that takes a bare infinitival either shares time with it on a cited ground or does
  not share it, and a judgment pins each case on a real card.
- Two time-giving dependents of one head that differ in time have no Reading together, whatever
  the head carries.
- `cargo xtask english strata --check` passes, and `cargo test` and
  `cargo clippy --tests -- -D warnings` pass for each changed crate.

## Pointers

- `crates/english/grammar/english.fug` (`dependents TimeGiving`, `BareInf`).
- `crates/syntax/src/compile.rs` (`dependents`).
