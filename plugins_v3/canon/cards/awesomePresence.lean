/-- Awesome Presence -/
card := {
  name := "Awesome Presence",
  cost := [blue],
  types := [enchantment],
  subtypes := [aura],
  text := [
    enchant isCreature,
    gatedBy (scaledGenericMana (forEach 3 [isCreature, controlledBy they, inCombat blockerOf it]))
      any (block (enchanted (type creature)))
  ]
}
