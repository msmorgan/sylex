card := {
  name := "Academy Journeymage",
  cost := [4, blue],
  types := [creature],
  subtypes := [human, wizard],
  text := [
    onlyIfSo (costShift this_ (less 1)) (exists_ [wizard, youControl]),
    when (does thisCreature enter) (returnToHand (target [isCreature, anOpponentControls]))
  ],
  power := 3,
  toughness := 2
}
