/-- Sheltered Valley -/
card shelteredValley : Spelled :=
  { name := "Sheltered Valley", types := [.land],
    text :=
      [ static
          (replace (would thisLand enter)
            (instead
              (sequentially
                [ sacrifice
                    (each
                      (and
                        [ isPermanent, isOther, named "Sheltered Valley",
                          youControl ])),
                  putOntoBattlefield thisLand ]))),
        at_ (beginningOfYour .upkeep)
          (if_ := compareAmt (countOf (and [isLand, youControl])) .atMost 3)
          (gainLife 1),
        activated tapSymbol (addMana 1 (manaOf [[.colorless]])) ] }
