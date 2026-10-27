card mishrasFactory : Spelled :=
  { name := "Mishra's Factory", types := [.land],
    text :=
      [ activated tapSymbol (addMana 1 (manaOf [[.colorless]])),
        activated [generic 1]
          (establishThroughout (qualityChange thisLand .sets
              (bundle
                { characteristics :=
                  { types := [.artifact, .creature], subtypes := [assemblyWorker],
                    power := stat 2, toughness := stat 2 } }
                (some .land))) untilEndOfTurn),
        activated tapSymbol
          (get (target (and [isCreature, assemblyWorker]))
            (up 1) (up 1) untilEndOfTurn) ] }
