/-- Omniscience -/
card := {
  name := "Omniscience",
  cost := [7, blue, blue, blue],
  types := [enchantment],
  text := [altCost (allOf [isSpell, did you (cast which (from_ := yourHand)) thisObject])]
}
