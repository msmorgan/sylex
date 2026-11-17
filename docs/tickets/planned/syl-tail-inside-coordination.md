---
needs: [syl-tail-placement]
---

# A Tail printed inside one conjunct of a coordination has no placement

## Why

A Tail stands at the edge of its host phrase, outside the host's other trailing dependents or,
marked `inner`, before them (`docs/design/spelling.md` §3.2, §7). *Ulasht, the Hate Seed* prints
"Ulasht enters with a +1/+1 counter on it for each other red creature you control and a +1/+1
counter on it for each other green creature you control": each "for each" stands inside one
conjunct of the coordination under "with", not at an edge of the "enters" phrase, and its parse
is settled that way (one Derivation). Neither `forEach` nor `forEachInner` prints it, and
`spelling.md` §7 lists "a Tail between two of its host's adjuncts" as something the notation
cannot say.

## Done when

- *Ulasht, the Hate Seed* is authored and its line 1 is written by a term whose spelling prints
  the card's order, or `spelling.md` says why the phrase is not a Tail there and what it is.
- Every other in-scope unit that prints "for each" inside a conjunct is listed from the pool's
  parses (`data/derived/english-forests.sqlite`), with the term each takes.
- `spelling.md` §3.2 and §7 state the result.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check`, `cargo test -p sylex_spelling` and
  `cargo test -p xtask` pass.

## Not in scope

- Finding a Tail's host in a Reading: recovery's (`english-syl-term-recovery`).

## Pointers

`docs/design/spelling.md` §3.2, §7; `plugins/builtin/templates/amount/{forEach,forEachInner}.syl`;
`cargo xtask english forestwalk --card "Ulasht, the Hate Seed" --line 1`.
