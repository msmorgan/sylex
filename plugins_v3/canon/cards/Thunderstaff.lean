card := {
  name := "Thunderstaff",
  cost := [3],
  types := [artifact],
  text := [
    onlyWhile
      (replace (would (a isCreature) (dealDamage anyAmount you (some combat)))
         (instead (preventDamage (ofThatDamage 1))))
      (is thisArtifact isUntapped),
    activated (compound [[2], tapSymbol])
      (get (allOf [isCreature, isAttacking]) (up 1) (up 0) untilEndOfTurn)
  ]
}
