/-- Iymrith, Desert Doom -/
card := {
  name := "Iymrith, Desert Doom",
  cost := [3, blue, blue],
  supertypes := [legendary],
  types := [creature],
  subtypes := [dragon],
  text := [
    flying,
    onlyWhile (abilityGrant thisCreature (ward [4])) (is it isUntapped),
    whenever (does thisCreature (dealDamage (to := a isPlayer) (kind := some combat)))
      [draw 1, doIf (compareAmt (countOf (cardIn yourHand)) less 3) (draw theDifference)]
  ],
  power := 5,
  toughness := 5
}
