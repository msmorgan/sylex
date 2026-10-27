card thrivingBluff : Spelled := { body := ((singleFacedCard
    { name := "Thriving Bluff", types := [.land],
      text :=
        [ static (entersTapped thisLand),
          static (entersChoosingFrom thisLand .color (colorOtherThan .red)),
          activated tapSymbol (addMana 1 (ofTheChosenColor (some [.of .red]))) ] })) }
