---
needs: []
---

# Syl: beam me up and assimilate

## Why

The keyword ability beam me up [CR#702.196] and the keyword action assimilate [CR#701.72] are
declared in the vocabulary catalogs, but neither has a `.syl`
definition, so no card can mean them. Beam me up is a graveyard alternative cost that returns a
creature you control and exiles the spell as it leaves the stack [CR#702.196a]; "can't be beamed
up" forbids returning that creature for the cost [CR#702.196b]. Assimilating gains control and puts
a +1/+1 counter on the permanent, which becomes a Borg artifact creature, or puts a card from
another zone onto the battlefield that way [CR#701.72a..701.72b], with "assimilated with" linked
abilities [CR#701.72c].

## Done when

- `plugins/builtin/definitions/keyword_ability/` defines beam me up and
  `plugins/builtin/definitions/keyword_action/` defines assimilate, each meaning what its rule says.
- "can't be beamed up" and "has beam me up"/"the beam me up cost" read: Marooned and Scotty,
  Miracle Worker are read in `cargo xtask english strata`.
- A real card exercises each: a beam me up spell (Pure Energy) and an assimilating card (Borg
  Queen, Perfection Manifest).
- `cargo xtask dsl check` and `cargo xtask dsl fmt --check` pass.

## Pointers

- `plugins/builtin/definitions/keyword_ability/flashback.syl`, `escape.syl` (graveyard alternative
  costs)
- `crates/vocabulary/catalogs/keyword_abilities.ron`, `keyword_actions.ron`
