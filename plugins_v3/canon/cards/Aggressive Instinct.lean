card := {
  name := "Aggressive Instinct",
  cost := [1, green],
  types := [sorcery],
  text := [
    spell
      (dealDamageOwnPower (target [isCreature, youControl]) (target [isCreature, youDontControl]))
  ]
}
