card := {
  name := "Paradox Haze",
  cost := [2, blue],
  types := [enchantment],
  subtypes := [aura],
  text := [
    enchant isPlayer,
    at_ (nthOccurrence (nth 1) (beginningOfPossessed upkeep (enchanted player)) (some turn))
      (instruct (that player) (additionalStep upkeep none 1))
  ]
}
