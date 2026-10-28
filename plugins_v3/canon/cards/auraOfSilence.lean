card := {
  name := "Aura of Silence",
  cost := [1, white, white],
  types := [enchantment],
  text := [
    costShift
      (allOf [or [isArtifact, isEnchantment], isSpell, did yourOpponents (cast which) thisObject])
      (more 2),
    activated (sacrifice thisEnchantment) (destroy (target (or [isArtifact, isEnchantment])))
  ]
}
