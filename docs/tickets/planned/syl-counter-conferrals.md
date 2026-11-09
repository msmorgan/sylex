---
needs: [cleanup-step6-surface]
---

# Keyword and rule-bearing counters confer what the rules give them

## Why

Of the counter declarations in `plugins/builtin/definitions/counter/`, only `p1p1Counter`,
`m1m1Counter` and `flyingCounter` have `confers`. Every other keyword counter is a name and a
comment, though a keyword counter makes its bearer gain that keyword [CR#122.1b]. The counters
the rules give an effect state it only in a comment: a shield counter's replacement and prevention
effects [CR#122.1c], a stun counter's replacement effect [CR#122.1d], a finality counter's
replacement effect [CR#122.1h], a rad counter's inherent triggered ability [CR#122.1i,728.1] and a
hone counter's +1/+0 to the creature its Equipment is attached to [CR#122.1j]. A player with ten or
more poison counters loses the game [CR#122.1f,704.5c], but `poison.syl` has no `confers`, and no
state-based row under `plugins/builtin/rules/` writes it.

Written against the surface (`cleanup-step6-surface`).

## Open questions

- How `.syl` tells a per-counter conferral from a once-for-one-or-more one. `p1p1Counter.syl`
  says its conferral applies once per counter held, which fits hone [CR#122.1j]; shield, stun,
  finality and rad counters create one effect for "one or more" counters
  [CR#122.1c,122.1d,122.1h,122.1i].
- Whether the Two-Headed Giant team threshold [CR#704.6b], which `poison.syl`'s comment names, is
  written with the ten-counter loss.

## Done when

- Every keyword counter [CR#122.1b] (`firstStrikeCounter`, `doubleStrikeCounter`,
  `deathtouchCounter`, `decayedCounter`, `exaltedCounter`, `hasteCounter`, `hexproofCounter`,
  `indestructibleCounter`, `lifelinkCounter`, `menaceCounter`, `reachCounter`, `shadowCounter`,
  `trampleCounter`, `vigilanceCounter`) confers its keyword as `flyingCounter` does.
- `shieldCounter` [CR#122.1c], `stunCounter` [CR#122.1d], `finalityCounter` [CR#122.1h],
  `radCounter` [CR#122.1i,728.1] and `honeCounter` [CR#122.1j] confer the effect their rule
  states, applied per counter or once for one or more counters as settled.
- `poison` confers, or a state-based row writes, the ten-counter loss [CR#122.1f,704.5c].
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass;
  `cargo xtask cite check --list-noncompliant` is empty and `cargo xtask cite check` reports
  0 stale, after `cargo xtask cite bless` for newly cited rules.

## Not in scope

- The basic land subtypes' mana abilities: `syl-basic-land-mana-abilities`.

## Pointers

`plugins/builtin/definitions/counter/` (`flyingCounter.syl` and `p1p1Counter.syl` for the form),
`docs/decisions/semantics.md` §1 item 4.
