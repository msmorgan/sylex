card bondersEnclave : Spelled :=
  { name := "Bonders' Enclave", types := [.land],
    text :=
      [ activated tapSymbol (addMana 1 (manaOf [[.colorless]])),
        activatedOnlyIf (compound [[generic 3], tapSymbol]) (draw 1)
          (exists_ (and [isCreature, youControl,
                          hasPower .atLeast 4])) ] }
