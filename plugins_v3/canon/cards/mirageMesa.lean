card mirageMesa : Spelled :=
  { name := "Mirage Mesa", types := [.land], subtypes := [desert],
    text :=
      [ static (entersTapped thisLand),
        static (entersChoosing thisLand .color),
        activated tapSymbol (addMana 1 ofTheChosenColor) ] }
