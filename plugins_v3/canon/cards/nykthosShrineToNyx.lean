/-- Nykthos, Shrine to Nyx -/
card nykthosShrineToNyx : Spelled :=
  { name := "Nykthos, Shrine to Nyx", supertypes := [.legendary], types := [.land],
    text :=
      [ activated tapSymbol (addMana 1 (manaOf [[.colorless]])),
        activated (compound [[generic 2], tapSymbol])
          (sequentially
            [ choose (a (quality .color)),
              addMana (devotion you thatColor) ofTheChosenColor ])
                  ] }
