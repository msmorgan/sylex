---
needs: []
---
# Engine: four cost and search rules the engine gets wrong: unmatched colored cost reductions, gain-life costs under "can't gain life", the [CR#601.2h] payment tiers, and the stated-quality search floor in public zones

## Why

No current card hits these yet; the first card of each kind will.

1. **A colored or {C} reduction with no matching pip.** `reduce_symbol` (`crates/engine/src/cast.rs`)
   removes a matching `Specific` pip, and an unmatched one is silently dropped. When the cost has
   no mana of that type, the reduction applies to generic mana instead [CR#118.7b], and an excess
   over the matching pips also comes off generic [CR#118.7c,118.7d]. So {W}{U}{B}{R}{G} off a
   {3}{U} spell should leave {0}; the engine leaves {3}. The Khalni Hydra ruling adds that its {G}
   reduction also cuts an imposed generic tax, after taking the green. Core
   `CostChange::Reduce(Arc<[CostComponent]>)` cannot say "colored only", which Edgewalker's
   reduction needs.
2. **A gain-life cost under "can't gain life".** `activate.rs` treats a `ChangeLife` cost with
   `LifeOp::Up` or `LifeOp::Set` as always payable. While the player who would gain the life
   can't gain life, such a cost can't be paid [CR#119.7], and a set that raises a life total is a
   gain [CR#119.5]. Per the Invigorate ruling, the player who gains is the opponent named by the
   cost, and a replacement of the gain (Sulfuric Vortex) still lets the cost be paid
   [CR#118.11]. The sibling [CR#119.8] lock has a seam comment and the pin test
   `cant_pay_life_lock_is_a_documented_seam`; [CR#119.7] has neither.
3. **Payment tiers.** `payment/fulfill.rs` (`is_deferred`, `action_is_deferred`) defers every
   `Search` from a library, every `DrawCard`, and every `Move` from a library to a non-library
   zone, hand included. Only costs that involve random elements or move objects from the library
   to a public zone go to the second tier [CR#601.2h]. The hand is a hidden zone [CR#400.2], so a
   library-to-hand move with no random element belongs in the first tier.
4. **Stated-quality search floor.** The `Instruction::Search` arm in `resolve/effect.rs` floors
   the minimum to 0 for any search that names a quality, in every zone it searches. The doc
   comment of `search_is_bare_quantity` in `resolve/query.rs` makes the same claim. The exemption
   covers only "searching a hidden zone" [CR#701.23b], and its example has Splinter's controller
   required to find the Howling Mine in the graveyard, a public zone [CR#400.2]. The Eradicate
   and Splinter rulings say the same.

## Done when

- Core `CostChange` tells a colored-only reduction apart by a typed construct (an enum or a
  separate variant, not a `bool`). A colored or {C} reduction that finds no matching pip, or that
  exceeds the matching pips, reduces generic unless it is colored-only. Tests
  cover the {3}{U} case, a Khalni Hydra-shaped {G} reduction under a generic tax, and Edgewalker's
  colored-only reduction.
- A gain-life or raising-`Set` cost is unpayable while the player who would gain can't gain life,
  and payable under a replacement of the gain. If the deontic layer cannot yet express "can't gain
  life", the always-payable comment becomes a [CR#119.7,119.5] seam with a pin test beside
  `cant_pay_life_lock_is_a_documented_seam`.
- A library-origin `Move` or `Search` cost is deferred only when it moves cards to a public zone or
  the cost includes a shuffle or other random element. The code that decides `DrawCard`'s tier
  states in its doc comment whether an unknown top card counts as a random element.
- A search's lower bound is computed per zone: candidates in a hidden zone that match only a
  stated quality floor to 0, and public-zone candidates use the quantity's own bound. The
  `Search` arm comment and the `search_is_bare_quantity` doc are corrected, and a test is grounded
  in Splinter or Eradicate.

## Pointers

- `crates/engine/src/cast.rs` (`reduce_mana_components`, `reduce_symbol`)
- `crates/core/src/continuous.rs` (`CostChange`)
- `plugins/canon/cards/Edgewalker.syl`
- `crates/engine/src/activate.rs` (the `ChangeLife` payability arm, the [CR#119.8] seam)
- `crates/engine/src/payment/fulfill.rs`
- `crates/engine/src/resolve/effect.rs` (`Instruction::Search`), `crates/engine/src/resolve/query.rs`
