card := {
  name := "Power Artifact",
  cost := [blue, blue],
  types := [enchantment],
  subtypes := [aura],
  text := [
    enchant isArtifact,
    costShift (allOf [abilityHead anyActivated, abilityOf (enchanted artifact)]) (less 2 (some 1))
  ]
}
