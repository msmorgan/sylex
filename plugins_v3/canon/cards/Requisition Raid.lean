card := {
  name := "Requisition Raid",
  cost := [white],
  types := [sorcery],
  text := [
    spell
      (chooseSpree
         [
           { cost := [1], action := destroy (target isArtifact) },
           { cost := [1], action := destroy (target isEnchantment) },
           {
             cost := [1],
             action := putCounters 1 p1p1Counter (each [isCreature, controlledBy (target isPlayer)])
           }
         ])
  ]
}
