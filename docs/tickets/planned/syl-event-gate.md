---
needs: [cleanup-step6-spine, syl-deontic-verb-phrase-operators]
specs: [voice-and-object-phrases]
---

# `cargo xtask dsl check` enforces the event gate

## Why

An event pattern, a lookback or a replacement's event must name something that happens as one
event (`docs/design/semantics.md` §3.2), and nothing checks it: `cargo xtask dsl check` resolves
names and sorts and has no rule about which verb phrase stands under `does`, `did` or `would`.
With `sequentially` declared at `VerbPhrase`, a sequence of steps is well sorted there. The gate
also has to read through the words that now stand between the event word and the verb: `done`,
`be`, the Templates that state what happens to their subject, and a coordination (spec §8).

## Open questions

- **How the checker knows a head is event-capable.** No declaration says so, and no table lists
  the heads. `semantics.md` §3.2 says which are not: the control-flow words (`sequentially`,
  `doIf`, `doForEach`, `chooseModes`), compound actions, and `remove`, which names no Deed. A
  rule derived from signatures alone (a primitive at `VerbPhrase` with no `VerbPhrase` or
  `Clause` parameter) would admit `remove`, so the set is not derivable. Options: a table in
  `crates/xtask/src/dsl_check/schema.rs`, where the schemas `.syl` does not declare live until
  step 7's AST states them; or a field in each primitive's block. Recommendation: the table,
  each entry with the rule or card line that makes it an event.

## Done when

- The open question is answered.
- `cargo xtask dsl check` refuses a verb phrase under `does`, `did` or `would` that names no
  event. It admits an event-capable head, a keyword action's Definition, a `done` over either,
  `be` over that, a Template whose body is one of these (`die`, `becomeBlocked`, `untapSelf`,
  `dealIndirect`), and an `and` or `or` of these. A test shows each admitted form, and a negative
  test shows a `sequentially`, a `doIf` and `remove` refused under each of the three words.
- Every call under `plugins/` passes the gate; a site it refuses is rewritten as its card's
  Oracle line or its rule's text prints it, and the landing lists each.
- `docs/design/semantics.md` §3.2 and `docs/design/surface.md` §7.5 state the gate as the
  spec's §8 does: they no longer say each arm of an alternative is a whole event with its own
  subject, or that an `or` over verb phrases is a compound action the gate refuses.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Not in scope

- A `shared` over event phrases: `syl-shared-phrase-written-once`, which adds it to what the
  gate admits.
- What the deontic operators and `doesnt` admit: not designed (spec §11).
- A keyword ability's verb as an event ("evolves", "trains"): `syl-keyword-verb-events`.
- A Deed stated by its own declaration kind: `syl-explicit-deeds`.

## Pointers

- `docs/specs/voice-and-object-phrases.md` §8, §9
- `docs/design/semantics.md` §3.2; `docs/design/surface.md` §7.5
- `crates/xtask/src/dsl_check.rs`, `crates/xtask/src/dsl_check/schema.rs`,
  `crates/xtask/src/dsl_expand.rs`
- `plugins/builtin/primitives/event/does.syl`, `plugins/builtin/primitives/action/remove.syl`
