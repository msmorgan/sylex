/-- Mutavault -/
card mutavault : Spelled :=
  { name := "Mutavault", types := [.land],
    text :=
      [ activated tapSymbol (addMana 1 (manaOf [[.colorless]])),
        activated [generic 1]
          (establishThroughout (qualityChange thisLand .sets
              (bundle
                { characteristics := { types := [.creature], power := stat 2, toughness := stat 2 },
                  qualities := [withEveryType .creature] }
                (some .land))) untilEndOfTurn) ] }
