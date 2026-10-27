card unchartedHaven : Spelled :=
  { name := "Uncharted Haven", types := [.land],
    text :=
      [ static (entersTapped thisLand),
        static (entersChoosing thisLand .color),
        activated tapSymbol (addMana 1 ofTheChosenColor) ] }
