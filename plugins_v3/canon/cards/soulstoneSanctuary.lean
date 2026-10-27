/-- Soulstone Sanctuary -/
card soulstoneSanctuary : Spelled :=
  { name := "Soulstone Sanctuary", types := [.land],
    text :=
      [ activated tapSymbol (addMana 1 (manaOf [[.colorless]])),
        activated [generic 4]
          (establish
            (qualityChange thisLand .sets
              (bundle
                { characteristics :=
                  { types := [.creature], text := [vigilance],
                    power := stat 3, toughness := stat 3 },
                  qualities := [withEveryType .creature] }
                (some .land)))) ] }
