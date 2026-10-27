card thrivingMoor : Spelled := { body := ((singleFacedCard
    { name := "Thriving Moor", types := [.land],
      text :=
        [ static (entersTapped thisLand),
          static (entersChoosingFrom thisLand .color (colorOtherThan .black)),
          activated tapSymbol (addMana 1 (ofTheChosenColor (some [.of .black]))) ] })) }
