card := {
  name := "Megatherium",
  cost := [2, green],
  types := [creature],
  subtypes := [beast],
  text := [
    trample,
    when (does thisCreature enter)
      (doUnless (sacrifice thisCreature) (scaledGenericMana (forEach 1 (cardIn yourHand))))
  ],
  power := 4,
  toughness := 4
}
