card turnToFrog : Spelled :=
  { name := "Turn to Frog", cost := [generic 1, pip .blue], types := [.instant],
    text :=
      [ spell (establishThroughout (conjunction [ allAbilityLoss (target isCreature),
              qualityChange it .sets
                (bundle { characteristics := { colors := [.blue], subtypes := [frog] } }),
              getsBase it 1 1 ]) untilEndOfTurn) ] }
