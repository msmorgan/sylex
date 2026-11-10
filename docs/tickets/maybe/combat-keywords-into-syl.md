---
needs: []
---
# Move the combat keywords the engine implements into `.syl`

## Why

First strike, double strike, deathtouch, trample and banding change how combat damage is assigned
and dealt. First strike, double strike, trample and banding are Primitive Keyword Abilities (a
label primitive and a Template over `primitiveKeyword`), and deathtouch an ability with an empty
body that a rule row reads. Each can become a Definition once `.syl` has the primitives its rules
need. The engine implements first and double strike; it has no trample, banding or
deathtouch-lethal assignment rule yet. In order of difficulty:

1. **Deathtouch and trample, together.** Trample's excess is what remains once the blockers are
   assigned lethal damage [CR#702.19b], and deathtouch makes any nonzero amount lethal for that
   purpose [CR#702.2c]. Neither can move alone. Trample may be a damage result.
2. **First strike and double strike.** They add a combat damage step [CR#510.4,702.7b,702.4b],
   which needs a turn-part primitive.
3. **Banding and "bands with other".** A block spreads to the whole band [CR#702.22h], and banding
   swaps who assigns combat damage [CR#702.22j,702.22k]. Raging River also sorts potential
   blockers into groups before blocks, so the two may share primitives.

## Done when

- Each moved keyword is a Definition whose body states its rule, and its label primitive and
  Template, or its empty body, are gone. For first and double strike, the engine's special case
  (`any_first_or_double_striker` and the skipped first-strike damage step) is gone; trample and
  banding are enforced through their Definitions, not a new engine special case.
- The existing combat tests pass unchanged.

## Pointers

- `plugins/builtin/primitives/keywordLabel/` and `plugins/builtin/templates/ability/`
  (`firstStrike`, `doubleStrike`, `trample`, `banding`, `bandsWithOther`),
  `plugins/builtin/primitives/ability/primitiveKeyword.syl`,
  `plugins/builtin/definitions/keyword_ability/deathtouch.syl`
- `plugins/builtin/rules/sba/deathtouchDestruction.syl`
- `crates/engine/src/combat.rs`, `crates/engine/src/step/combat.rs`,
  `crates/engine/src/decide/pending/combat.rs` (`AssignCombatDamage`)
- `docs/tickets/planned/syl-deathtouch-creature-conferral.md`
