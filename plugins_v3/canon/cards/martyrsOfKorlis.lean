card martyrsOfKorlis : Spelled :=
  { name := "Martyrs of Korlis", cost := [generic 3, pip .white, pip .white],
    types := [.creature], subtypes := [human],
    text :=
      [ static (onlyWhile
          (replace (would (by_ (allOf isArtifact)) (dealDamage anyAmount you))
            (instead (redirectDamage allDamage thisCreature)))
          (is thisCreature isUntapped)) ],
    power := stat 1, toughness := stat 6 }
