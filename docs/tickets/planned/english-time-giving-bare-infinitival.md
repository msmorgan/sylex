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

## Design

- Keep `BareInf` in `dependents TimeGiving` (documenting modals, supportive *do*, causative *have*
  [CGEL, Ch. 14, §5.4, p. 1236; §5.6.2, p. 1244], *rather than* [Ch. 15, §2.8, p. 1317], and cost
  *plus*), and add `PastPart` (`have + NP + PastPart`) to `dependents TimeGiving` with `Time`.
- Collapse `PastTime` and `NonPastTime` into a single `feature Time = Past | Mixed | NonPast` with
  `resolve: [Time -> Mixed]` on coordinations, so non-coordination `TimeGiving` unifies `$.Time`
  across any number of dependents without pairwise `ExcludeTogether` negations.

## Done when

- `BareInf` and `PastPart` share time through `dependents TimeGiving` with their comment citing all
  governors, and judgments pin causative *have* (`BareInf` and `PastPart`) and *rather than*.
- `PastTime` and `NonPastTime` are replaced by `feature Time = Past | Mixed | NonPast` with
  `resolve: [Time -> Mixed]`, and two time-giving dependents of one head that differ in time have no
  Reading together whatever the head carries.
- `cargo xtask english strata --check` passes, and `cargo test` and
  `cargo clippy --tests -- -D warnings` pass for each changed crate.

## Pointers

- `crates/english/grammar/english.fug` (`feature PastTime`, `feature NonPastTime`,
  `dependents TimeGiving`, `BareInf`, `PastPart`, `PastParticipial`, time-bearing `coordination`
  schemas).
- `crates/syntax/src/compile.rs` (`dependents`, `resolve`).
