/-- Yedora, Grave Gardener -/
card yedoraGraveGardener : Spelled :=
  { name := "Yedora, Grave Gardener", cost := [generic 4, pip .green],
    supertypes := [.legendary], types := [.creature],
    subtypes := [treefolk, druid],
    text :=
      [ whenever
          (does (a (and [isNontoken, and [isCreature, youControl], isOther])) die)
          (may
            (sequentially
              [ returnToBattlefield it [(faceDownAs
                      { characteristics := some { types := [.land], subtypes := [forest] }
                          }), (under (ownerOf it))] ])) ],
    power := stat 5, toughness := stat 5 }
