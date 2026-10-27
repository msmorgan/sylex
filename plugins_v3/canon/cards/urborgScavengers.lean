card urborgScavengers : Spelled :=
  { name := "Urborg Scavengers", cost := [generic 2, pip .black], types := [.creature],
    subtypes := [spirit],
    text :=
      [ whenever (does thisCreature enter) (or_ := [does thisCreature attack])
          (sequentially
            [ exile (target (cardIn graveyard)),
              putCounters 1 p1p1Counter thisCreature ]),
        alsoForKeywords
          (fun k => static (onlyWhile
            (keywordGrant thisCreature k (and [in_ (exiledWith thisCreature), hasKeyword k]))
            (exists_ (and [in_ (exiledWith thisCreature), hasKeyword k]))))
          [ flying, firstStrike, doubleStrike, deathtouch, haste, hexproofAbilities,
            indestructible, lifelink, menace, reach, trample, vigilance ] ],
    power := stat 2, toughness := stat 2 }
