card thrivingGrove : Spelled := { body := ((singleFacedCard
    { name := "Thriving Grove", types := [.land],
      text :=
        [ static (entersTapped thisLand),
          static (entersChoosingFrom thisLand .color (colorOtherThan .green)),
          activated tapSymbol (addMana 1 (ofTheChosenColor (some [.of .green]))) ] })) }
