card := {
  name := "Talion, the Kindly Lord",
  cost := [2, blue, black],
  supertypes := [legendary],
  types := [creature],
  subtypes := [faerie, noble],
  text := [
    flying,
    entersChoosing thisCreature number,
    whenever
      (does anOpponent
         (cast
            (a
               [
                 isSpell,
                 compare eq chosenNumber (axes := [stat manaValue, stat power, stat toughness])
               ])))
      [instruct (that player) (loseLife 2), draw 1]
  ],
  power := 3,
  toughness := 4
}
