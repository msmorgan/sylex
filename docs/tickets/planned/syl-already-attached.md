---
needs: [syl-action-to-verb, cleanup-step6-surface]
---

# "Already attached": the permanents attached when an effect starts to apply

## Why

Two cards print "already attached" in the object of "This effect doesn't remove …": *Benevolent
Blessing* ("Auras and Equipment you control that are already attached to it") and *Guardian
Beast* ("Auras already attached to those artifacts"). For *Benevolent Blessing* the rules say
what it means: the permanents attached to the creature when the protection effect starts to
apply keep their place, and others can't become attached [CR#702.16p]. `plugins/builtin/` has
no word for "attached when this effect started to apply": `attachedTo` reads the present, and
`heldSince` is a Condition over a GameEvent, and the start of an effect is no GameEvent. Both
cards write the object without "already" and say so in a "Fragment" comment.

## Open questions

- What word says "already attached": the permanents attached when an effect starts to apply.
- Whether *Guardian Beast*'s "already" means "attached when the effect starts to apply", which
  [CR#702.16p] defines for *Benevolent Blessing* only.

## Done when

- *Benevolent Blessing* and *Guardian Beast* write "already attached" in the object of their
  `remove`, and their "Fragment" comments are gone.
- Declaration order stays the printed order (`syl.md` §5).
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Pointers

`plugins/canon/cards/Benevolent Blessing.syl`, `plugins/canon/cards/Guardian Beast.syl`,
`plugins/builtin/templates/predicate/attachedTo.syl`,
`plugins/builtin/primitives/condition/heldSince.syl`.
