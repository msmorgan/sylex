card thrivingHeath : Spelled := { body := ((singleFacedCard
    { name := "Thriving Heath", types := [.land],
      text :=
        [ static (entersTapped thisLand),
          static (entersChoosingFrom thisLand .color (colorOtherThan .white)),
          activated tapSymbol (addMana 1 (ofTheChosenColor (some [.of .white]))) ] })) }
