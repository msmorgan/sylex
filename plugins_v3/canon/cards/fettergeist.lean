card := {
  name := "Fettergeist",
  cost := [2, blue],
  types := [creature],
  subtypes := [spirit],
  text := [
    flying,
    at_ (beginningOfYour upkeep)
      (doUnless (sacrifice thisCreature)
         (scaledGenericMana (forEach 1 [isCreature, youControl, isOther])))
  ],
  power := 3,
  toughness := 4
}
