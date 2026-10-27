/-- Omniscience -/
card omniscience : Spelled :=
  { name := "Omniscience", cost := [generic 7, pip .blue, pip .blue, pip .blue],
    types := [.enchantment],
    text :=
      [ static (altCost (allOf (and [isSpell,
          did you (cast which (from_ := yourHand)) thisObject]))) ] }
