---
needs: []
---
**Block legality is proven for every evasion keyword, not only flying and menace.**

## Why

Block legality evaluates `Cant(Block)` rows generically, point-wise and arrangement-level, and
flying, reach, menace and defender have engine tests. The other evasion keywords are declared as
`Cant(Block)` rows over blocker predicates the engine has never been tested against: color and
type (fear), shared color with the attacker (intimidate), the keyword on both sides (shadow,
horsemanship), compared power (skulk), what the defending player controls (landwalk), and
protection's blocking clause. Any of those predicates may not evaluate correctly in the block
check.

## Done when

- An engine test per keyword, each with a real card, asserts both a legal and an illegal block:
  Severed Legion (fear) [CR#702.36b], Bladetusk Boar (intimidate) [CR#702.13b], Soltari Priest
  (shadow, both directions) [CR#702.28b], Lu Xun, Scholar General (horsemanship) [CR#702.31b],
  Furtive Homunculus (skulk) [CR#702.118b], Bog Wraith (swampwalk) [CR#702.14c], and White Knight
  (protection from black) [CR#702.16f]. Use a core fixture where a card is not authored.
- Whatever those tests expose in block legality or predicate evaluation is fixed in the engine.

## Pointers

- `crates/engine/src/legal.rs`: `legal_blockers`, `cant_block_rows`, `block_forbidden_by`.
- `crates/engine/tests/combat.rs`: the menace tests.
- `plugins/builtin/definitions/keyword_ability/`: `fear.syl`, `intimidate.syl`, `shadow.syl`,
  `horsemanship.syl`, `skulk.syl`, `landwalk.syl`, `protection.syl`.
