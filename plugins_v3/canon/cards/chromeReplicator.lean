card chromeReplicator : Spelled :=
  { name := "Chrome Replicator", cost := [generic 5], types := [.artifact, .creature],
    subtypes := [construct],
    text :=
      [ when (does thisCreature enter)
          (if_ := thereAre (withTheSameName
            (counted (orMore 2)
              (and [isPermanent, non isLand, isNontoken, youControl]))))
          (create 1 (written
            { characteristics :=
              { types := [.artifact, .creature], subtypes := [construct],
                power := stat 4, toughness := stat 4 } })) ],
    power := stat 4, toughness := stat 4 }
