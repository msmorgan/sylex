card ancientZiggurat : Spelled :=
  { name := "Ancient Ziggurat", types := [.land],
    text :=
      [ activated tapSymbol
          (addMana 1 (ofAnyColor .sameColor)
            [spendOnly [toCast (and [isCreature, isSpell])]]) ] }
