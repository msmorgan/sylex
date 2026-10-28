card := {
  name := "Raging Ravine",
  types := [land],
  text := [
    entersTapped thisLand,
    activated tapSymbol (addMana 1 (manaOf [[of red], [of green]])),
    activated [2, red, green]
      (establishThroughout
         (qualityChange thisLand sets
            (bundle
               {
                 colors := [red, green],
                 types := [creature],
                 subtypes := [elemental],
                 text := [whenever (does thisCreature attack) (putCounters 1 p1p1Counter thisCreature)],
                 power := 3,
                 toughness := 3
               }
               (some land)))
         untilEndOfTurn)
  ]
}
