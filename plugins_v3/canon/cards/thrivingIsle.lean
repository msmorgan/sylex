card thrivingIsle : Spelled := { body := ((singleFacedCard
    { name := "Thriving Isle", types := [.land],
      text :=
        [ static (entersTapped thisLand),
          static (entersChoosingFrom thisLand .color (colorOtherThan .blue)),
          activated tapSymbol (addMana 1 (ofTheChosenColor (some [.of .blue]))) ] })) }
