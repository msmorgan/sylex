card ragingRavine : Spelled :=
  { name := "Raging Ravine", types := [.land],
    text :=
      [ static (entersTapped thisLand),
        activated tapSymbol (addMana 1 (manaOf [[.of .red], [.of .green]])),
        activated [generic 2, pip .red, pip .green]
          (establishThroughout (qualityChange thisLand .sets
              (bundle
                { characteristics :=
                  { colors := [.red, .green], types := [.creature],
                    subtypes := [elemental],
                    text :=
                      [ whenever (does thisCreature attack)
                          (putCounters 1 p1p1Counter thisCreature) ],
                    power := stat 3, toughness := stat 3 } }
                (some .land))) untilEndOfTurn) ] }
