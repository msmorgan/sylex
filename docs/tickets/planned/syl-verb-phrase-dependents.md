---
needs: [syl-kind-sorts-and-position-unions]
specs: [voice-and-object-phrases]
---

# Depictives and adverbials are the verb phrase's dependents, not `MoveRider`

## Why

"Put it onto the battlefield tapped and attacking", "enters tapped" and "attacks alone" end in a
phrase that describes the subject or object as the event happens, which the English grammar reads
as a `Depictive`. The term writes the ones on a move as a `riders` list at the sort `MoveRider`,
a sort that names what a phrase does to a move and no kind of phrase (`docs/design/surface.md`
§1), and the others as Templates of their own. `MoveRider` holds its own copies of words the
description sorts already have: `attachedTo`, `not`, `entersAs(status)` for "tapped" and "face
down", `entersTransformed`, `entersAttacking`, `underControlOf`. Thirty canon sites in 27 cards
write a `riders` list.

`docs/design/semantics.md` §6 item 13 and `docs/specs/voice-and-object-phrases.md` §13 item 1
state the direction and design neither: a depictive is a dependent of its verb phrase, built from
the description words, and the verb phrase's unselected dependents (a duration, a window, "this
way", "for each", the agent, a depictive) stand in one list in printed order, as a description's
modifiers do, with the verb's selected complements staying its own parameters.

## Open questions

- Which dependents the list holds and which stay parameters of their verb, decided from the
  parses of the units that print each (the spec's examples: an agent before and after another
  dependent, a trailing phrase after a modal, "for each" before and after a duration).
- Whether the depictive position is a union of the kinds of phrase beside `PreModifier`,
  `PostModifier` and `Predicative`, and which kinds it admits ("tapped", an adjective phrase;
  "face down", a preposition phrase; "attacking", a participle; "under your control"; "with a
  +1/+1 counter on it").
- What the riders that are no phrase of a kind become: `asCopyOf`, `withCharacteristicEdits`,
  `withCounters`'s mark, `inOrder`, `entersMelded`.

## Done when

- The open questions are answered in `voice-and-object-phrases.md` and `semantics.md` §6 item 13
  is no longer open.
- A verb phrase's unselected dependents are written in one list in printed order, each a phrase
  of its kind declared once, and every canon `riders` site and every depictive Template is
  respelled to it, each checked against its card's parse.
- `MoveRider` is gone, with `withRiders` and each declaration that took a `List(MoveRider)`
  restated over the dependents, and no word is declared at a second sort for standing after a
  verb.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check`, `cargo test -p xtask` and
  `cargo clippy -p xtask --tests -- -D warnings` pass.

## Not in scope

- The Position Phrase after a zone ("second from the top"): `syl-zone-words`.
- What a dependent means on each verb (an Entry Rider on an entering), which semantics reads.

## Pointers

- `plugins/builtin/primitives/zone/` (the `MoveRider` words), `plugins/builtin/primitives/move/`,
  `plugins/builtin/primitives/static/withRiders.syl`, `plugins/builtin/primitives/action/move.syl`,
  `plugins/builtin/templates/action/` and `templates/ability/` (the declarations with `riders`)
- `crates/english/grammar/english.fug` (`Depictive`, `PredicateAdjunct`)
- `docs/specs/voice-and-object-phrases.md` §8, §13; `docs/design/semantics.md` §6 item 13;
  `docs/design/spelling.md` §3.2
