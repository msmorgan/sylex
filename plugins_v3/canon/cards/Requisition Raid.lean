/-- Requisition Raid -/
card := {
  name := "Requisition Raid",
  cost := [white],
  types := [sorcery],
  text := [
    spell
      (chooseSpree
         [
           ([1], destroy (target isArtifact)),
           ([1], destroy (target isEnchantment)),
           ([1], putCounters 1 p1p1Counter (each [isCreature, controlledBy (target isPlayer)]))
         ])
  ]
}
