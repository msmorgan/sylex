card infernalVessel : Spelled :=
  { name := "Infernal Vessel", cost := [generic 2, pip .black], types := [.creature],
    subtypes := [human, cleric],
    text :=
      [ when (does thisCreature die) (if_ := some (not (is it demon)))
          (sequentially
            [ returnToBattlefieldWithCounters it (ownerOf it) 2 p1p1Counter,
              become it { characteristics := { subtypes := [demon] } } ]) ],
    power := stat 2, toughness := stat 1 }
