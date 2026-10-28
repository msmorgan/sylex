/-- Stuffy Doll -/
card := {
  name := "Stuffy Doll",
  cost := [5],
  types := [artifact, creature],
  subtypes := [construct],
  text := [
    indestructible,
    entersChoosingPlayer thisCreature (a isPlayer),
    whenever (does any (dealDamage (to := thisCreature)))
      (instruct it (dealDamage thatMuch (the isChosenPlayer))),
    activated tapSymbol (instruct thisCreature (dealDamage 1 thisCreature))
  ],
  power := 0,
  toughness := 1
}
