---
needs: [cleanup-step6-surface, syl-declaration-rules-field, syl-counters-noun-phrase]
specs: [confers-and-rules]
---

# Keyword and rule-bearing counters confer what the rules give them

## Why

Of the counter declarations in `plugins/builtin/definitions/counter/`, only `p1p1Counter`,
`m1m1Counter` and `flyingCounter` have `confers`. Every other keyword counter is a name and a
comment, though a keyword counter makes its bearer gain that keyword [CR#122.1b]. The counters
the rules give an effect state it only in a comment: a shield counter's replacement and prevention
effects [CR#122.1c], a stun counter's replacement effect [CR#122.1d], a finality counter's
replacement effect [CR#122.1h], a rad counter's inherent triggered ability [CR#122.1i,729.1] and a
hone counter's +1/+0 to the creature its Equipment is attached to [CR#122.1j]. A player with ten or
more poison counters loses the game [CR#122.1f,704.5c], but `poison.syl` has no `confers`, and no
state-based row under `plugins/builtin/rules/` writes it.

Written against the surface (`cleanup-step6-surface`).

## Design

- A per-counter effect stays in a kind's `confers`: the keyword kind's keyword [CR#122.1b], which
  `syl-counters-noun-phrase` writes, and hone [CR#122.1j]. A once-for-one-or-more effect goes in its `rules`, scoped with
  `hasCounter(<kind>)`: shield [CR#122.1c], stun [CR#122.1d], finality [CR#122.1h], rad
  [CR#122.1i,729.1] and the ten-poison loss [CR#122.1f,704.5c] (spec §3).

## Open questions

- Whether the Two-Headed Giant team threshold [CR#704.6b], which `poison.syl`'s comment names, is
  written with the ten-counter loss.

## Done when

- The keyword kind confers its keyword [CR#122.1b] and `boost` its numbers [CR#122.1a], each per
  counter, as `syl-counters-noun-phrase` leaves them; no keyword counter has a declaration of its
  own.
- `shieldCounter` [CR#122.1c], `stunCounter` [CR#122.1d], `finalityCounter` [CR#122.1h],
  `radCounter` [CR#122.1i,729.1] and `honeCounter` [CR#122.1j] write the effect their rule
  states, hone in `confers` and the others in `rules`.
- `poison` writes the ten-counter loss in `rules` [CR#122.1f,704.5c].
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass;
  `cargo xtask cite check --list-noncompliant` is empty and `cargo xtask cite check` reports
  0 stale, after `cargo xtask cite bless` for newly cited rules.

## Not in scope

- The basic land subtypes' mana abilities: `syl-basic-land-mana-abilities`.

## Pointers

`plugins/builtin/definitions/counter/` (`flyingCounter.syl` and `p1p1Counter.syl` for the form),
`docs/specs/confers-and-rules.md`, `docs/design/semantics.md` §1 item 4.
