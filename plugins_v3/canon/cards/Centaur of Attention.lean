card := {
  name := "Centaur of Attention",
  cost := [3, green, green],
  types := [creature],
  subtypes := [centaur, performer],
  text := [
    when (does thisCreature enter) [rollDice 5 (sides 6), storeResults thisCreature],
    at_ (beginningOfYour combat) (may (rerollStored anyNumber thisCreature)),
    static
      [
        getsPt thisCreature (up (letter x)) (up (letter x)),
        letterDefinition x (greatestStoredMatch thisCreature)
      ]
  ],
  power := 3,
  toughness := 3
}
