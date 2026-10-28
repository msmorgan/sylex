/-- Deadpool, Trading Card -/
card := {
  name := "Deadpool, Trading Card",
  cost := [2, black, red],
  supertypes := [legendary],
  types := [creature],
  subtypes := [mutant, mercenary, hero],
  text := [
    replace (would thisCreature enter)
      (alongside (may (exchangeTextBoxes thisCreature (a [isCreature, isOther])))),
    at_ (beginningOfYour upkeep) (loseLife 3),
    activated (compound [[3], sacrifice thisCreature]) (instruct (each isOtherPlayer) (draw 1))
  ],
  power := 5,
  toughness := 3
}
