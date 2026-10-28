card := {
  name := "Daughter of Autumn",
  cost := [2, green, green],
  supertypes := [legendary],
  types := [creature],
  subtypes := [avatar],
  text := [
    activated [white]
      (establishThroughout
         (replace (would any (dealDamage anyAmount (target [isCreature, hasColor white])))
            (instead (redirectDamage (theNext 1) thisCreature)))
         thisTurn)
  ],
  power := 2,
  toughness := 4
}
