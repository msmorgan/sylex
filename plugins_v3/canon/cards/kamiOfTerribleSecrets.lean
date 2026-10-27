card kamiOfTerribleSecrets : Spelled :=
  { name := "Kami of Terrible Secrets", cost := [generic 3, pip .black], types := [.creature],
    subtypes := [spirit],
    text :=
      [ when (does thisCreature enter)
          (if_ := some (and [ exists_ (and [isArtifact, youControl]),
                  exists_ (and [isEnchantment, youControl]) ]))
          (sequentially [draw 1, gainLife 1]) ],
    power := stat 3, toughness := stat 4 }
