card := {
  name := "Irenicus's Vile Duplication",
  cost := [3, blue],
  types := [sorcery],
  text := [spell (create 1 (copyOf (target [isCreature, youControl]) [alsoHas flying, isntLegendary]) [])]
}
