---
needs: [syl-single-face-card-call]
---

# "This effect doesn't remove …": an effect as a verb's Subject

## Why

Fifteen canon cards print "This effect doesn't remove <attachments>" after a static ability's
effect: fourteen protection Auras [CR#702.16n,702.16p] and *Guardian Beast*, which grants no
protection. `.syl` writes the sentence as `retention(spec, subject)`, a wrapper around one clause
of the effect (`plugins/builtin/primitives/static/retention.syl`). That is wrong: the sentence's
subject is the whole effect, it is a sentence of its own, and which of the effect's consequences
it exempts (an illegally attached Aura going to the graveyard [CR#704.5m], Equipment becoming
unattached [CR#704.5n]) is the engine's question.

An Action is already the verb phrase of every verb (`semantics.md` §4.1). What the sentence lacks
is narrower: an effect as a verb's Subject, where every Subject so far names a player or an
object, and a plain negative declarative among the contexts a verb stands in.

## Design

- `thisEffect` is the NounPhrase "this effect": the effect of the enclosing spell or ability,
  found by enclosure as being an effect is (`semantics.md` §4.7), never by a pronoun search.
- `doesnt(subject, action)` is the StaticSpec "<subject> doesn't <action>": what its Subject does
  not do. It is no deontic rule and takes no deontic modifiers.
- `remove(what)` is the Action "remove <what>". It names no Deed and happens as no one event, so
  the Event Gate refuses it as an event, a lookback or a prohibition (`semantics.md` §4.6).
- The sentence is written `doesnt(actor(thisEffect), remove(…))`, its own Static Spec beside the
  spec it talks about, the two joined by `conjunction` in one static ability, so its references
  ("those artifacts") reach the first sentence's referents. Its object is what the card prints.
- `.syl` does not say which consequence of the effect the sentence exempts.

## Done when

- `retention` is gone, and each of its 15 callers writes "This effect doesn't remove …" as
  `doesnt(actor(thisEffect), remove(…))` beside its effect, with the object its Oracle text prints.
- `semantics.md` and the game-model glossary state the new context and who may be a Subject.
- Declaration order stays the printed order (`syl.md` §5).
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Not in scope

- "Already" in *Benevolent Blessing*'s and *Guardian Beast*'s objects: `syl-already-attached`.
- The other "this effect …" sentences (*Power Artifact*'s floor, *Edgewalker*'s colored-only
  reduction), written today as typed parameters of the cost change.

## Pointers

`plugins/builtin/primitives/static/retention.syl`, `plugins/canon/cards/Guardian Beast.syl`,
`docs/decisions/semantics.md` §4.
