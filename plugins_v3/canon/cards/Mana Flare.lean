card := {
  name := "Mana Flare",
  cost := [2, red],
  types := [enchantment],
  text := [
    whenever (does (a isPlayer) (tapForMana (a isLand)))
      (instruct (that player) (addMana 1 (ofTypeProducedBy (that land))))
  ]
}
