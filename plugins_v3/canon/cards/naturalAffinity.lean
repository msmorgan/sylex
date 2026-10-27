card naturalAffinity : Spelled :=
  { name := "Natural Affinity", cost := [generic 2, pip .green], types := [.instant],
    text :=
      [ spell (establishThroughout (qualityChange (allOf isLand) .sets
            (bundle { characteristics := { types := [.creature], power := stat 2, toughness := stat 2 } }
              (some .land))) untilEndOfTurn) ] }
