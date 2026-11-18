---
needs: []
---

# `SetNameLicense` gates a wording no card prints

## Why

`feature SetNameLicense = Head` on *expansion* and its two gates (`NamePremodifier` and
`NominalPremodifier` refuse a head that carries it) keep a card name or a noun premodifier from
standing before *expansion* beside the Set Name. They change a Reading only where a card cites a
set whose name is also a card name or a subtype, in that card's own text (*in the Time Spiral
expansion* on *Time Spiral*). The sets cards cite are Antiquities, Homelands and Arabian Nights,
so the gates change no Corpus Unit (`docs/design/english.md` §6).

## Done when

- `SetNameLicense`, its two gate clauses and its paragraph in `docs/design/english.md` §3 are
  removed, and `crates/english/tests/set_names.rs` asserts that the Set Name Reading is among a
  line's Readings, not that it is the only one.
- The census is unchanged, and `cargo xtask english strata --check`, `cargo test` and
  `cargo clippy --tests -- -D warnings` pass for every changed crate.

## Pointers

- `crates/english/grammar/english.fug`: `feature SetNameLicense`, `NamePremodifier`,
  `NominalPremodifier`; `crates/english/tests/set_names.rs`.
