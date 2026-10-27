card thunderstaff : Spelled :=
  { name := "Thunderstaff", cost := [generic 3], types := [.artifact],
    text :=
      [ static (onlyWhile
          (replace (would (a isCreature) (dealDamage anyAmount you (some .combat)))
            (instead (preventDamage (ofThatDamage 1))))
          (is thisArtifact isUntapped)),
        activated (compound [[generic 2], tapSymbol])
          (get (allOf (and [isCreature, isAttacking])) (up 1) (up 0)
            untilEndOfTurn) ] }
