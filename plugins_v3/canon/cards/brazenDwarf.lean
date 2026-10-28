card := {
  name := "Brazen Dwarf",
  cost := [1, red],
  types := [creature],
  subtypes := [dwarf, shaman],
  text := [
    whenever (does you (rollDice anyAmount anyDie))
      (instruct thisCreature (dealDamage 1 eachOpponent))
  ],
  power := 1,
  toughness := 3
}
