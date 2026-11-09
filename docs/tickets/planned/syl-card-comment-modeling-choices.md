---
needs: [cleanup-step6-surface]
---

# Canon card comments state the card's modeling choice

## Why

Eight canon cards open with a comment that begins "Pins …": *Chance Encounter*, *Cursed Scroll*,
*Do or Die*, *Exploration*, *Glaring Spotlight*, *Jace Beleren*, *Mana Leak* and *Tribal Flames*
(`plugins/canon/cards/`). Each describes what a deleted Lean pin checked ("Pins the unless-payment
construction with the targeted spell’s controller as the payer"), not how the card is written or
why. Nothing pins anything now, so a reader is told about a test that does not exist.

## Done when

- Each of the eight comments states the card's modeling choice: which construct the card is
  written with and why, keeping the rule each choice rests on cited. No canon card comment opens
  "Pins" or describes a check.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass;
  `cargo xtask cite check --list-noncompliant` is empty and `cargo xtask cite check` reports
  0 stale, after `cargo xtask cite bless` for newly cited rules.

## Pointers

The eight card files in `plugins/canon/cards/`.
