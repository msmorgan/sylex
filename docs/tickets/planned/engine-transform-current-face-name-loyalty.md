---
needs: [engine-transform]
---
# Read a transformed permanent's current face everywhere

## Why

A double-faced permanent with its back face up has only its back face's characteristics, except
that its mana value uses the front face's mana cost [CR#712.8e]. `engine-transform` made
`base_values`, `abilities_of_source` and the player statics read the current face
(`derive::face_of`), but other battlefield reads still use `derive::face`, which is always the
front face:

- the legend rule's name key (`sba.rs`): the legendary check uses the layered view, the name it
  groups by is the front face's [CR#704.5j];
- the `Named` and `NamedReg` predicates (`target.rs`);
- the printed-loyalty base reads (`target.rs`, `activate.rs`, `layer.rs`), which matter for a
  double-faced card whose back face is a planeswalker;
- `distinct_keys_derived` (`layer.rs`): types, subtypes, basic land types, supertypes and name;
- `copiable_values` (`copy.rs`): a copy of a double-faced permanent uses the copiable values of the
  face that's up [CR#707.8], and a token copy of one is a double-faced token that enters back face
  up when the original's back face was up [CR#707.8a]. `CopiableValues` carries one face and no
  side.

The layered view (`DerivedObject`, `layer.rs`) has no name, so the legend rule and `Named` also
ignore copy effects: a Clone copying a legend is grouped by "Clone". The legend rule also compares
names as exact strings, where two objects have the same name if they have at least one name in
common [CR#201.2a].

Delver of Secrets transforms, so a back-face-up permanent is reachable, but no current card
exercises these reads.

## Done when

- The layered view carries the name, derived from the current face and copy effects, and the
  legend rule, `Named`, `NamedReg` and `distinct_keys_derived` read it and the other current-face
  characteristics.
- The legend rule groups by name-set intersection [CR#201.2a,704.5j].
- Printed-loyalty reads of a battlefield object use its current face.
- Copying a double-faced permanent copies the face that's up; a token copy carries both faces and
  enters with the original's side up [CR#707.8,707.8a]. For example, Cackling Counterpart on a
  transformed Delver of Secrets makes a double-faced token, back face up.
- Reads off the battlefield and off the stack, cast-time reads and mana value stay on the front
  face [CR#712.8a,712.8e].
- Tests cover each read with a back-face-up permanent: a real double-faced card, or a minimal
  repro derived from one.

## Pointers

- `crates/engine/src/derive.rs` (`face`, `face_of`)
- `rg -n 'derive::face\(' crates/engine/src` for the front-face reads
- `crates/engine/src/layer.rs` (`DerivedObject`, `base_values`, `distinct_keys_derived`),
  `crates/engine/src/sba.rs`, `crates/engine/src/target.rs`, `crates/engine/src/activate.rs`,
  `crates/engine/src/copy.rs`, `crates/core/src/copy.rs` (`CopiableValues`)
