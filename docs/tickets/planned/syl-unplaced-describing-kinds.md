---
needs: [syl-union-membership]
specs: [kind-sorts-and-position-unions]
---

# Describing phrases no position sort admits

## Why

The pool prints five kinds of phrase around a head or after a copula that are members of no
position sort (units are Corpus Units with some Derivation that has the phrase in that function):

| Position | Phrase | Units |
|---|---|---|
| before a head | a keyword ("Equip abilities", "flashback cost") | 163 |
| before a head | a power/toughness measure outside tokens and counters ("Target 1/1 creature", *Aegis of the Meek*) | 12 |
| before a head | a name ("Wicked Role token") | 69 |
| before a head | mana symbols ("{2}{U} kicker") | 30 |
| after a head | a temporal noun phrase ("the first time each turn") | 131 |
| after a copula | a measure ("except it's 1/1", *Endless Evil*) | 23 |

## Open questions

- **Which of these are modifiers.** A unit counts if any Derivation has the phrase there, so
  each row is an upper bound: for each, read the settled parses and say whether the phrase is a
  dependent of the head, and what the cards that print it write today.
- **The sort each stands at**, and whether it is a member of the position sort or a wording of
  the head it is printed with ("flashback cost" as a noun of its own). A keyword and a measure
  before "counter" are `syl-counters-noun-phrase`'s, and a measure after "it's" in a copy
  exception `syl-copy-exceptions`'s.

## Done when

- The open questions are answered with the parses that decide them, each answer in the doc
  comment of the declaration it shapes.
- Every phrase that is a modifier has a declaration at its kind and a membership, and one card
  that prints it is authored or rewritten for each.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Pointers

`docs/specs/kind-sorts-and-position-unions.md` §5; `crates/english/README.md` (the forest
database).
