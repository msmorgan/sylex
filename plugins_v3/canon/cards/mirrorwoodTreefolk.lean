card := {
  name := "Mirrorwood Treefolk",
  cost := [3, green],
  types := [creature],
  subtypes := [treefolk],
  text := [
    activated [2, red, white]
      (establishThroughout
         (replace (would any (dealDamage anyAmount thisCreature))
            (instead (redirectDamage allDamage (target isAnyTarget))) (use := nextTimeOnly))
         thisTurn)
  ],
  power := 2,
  toughness := 4
}
