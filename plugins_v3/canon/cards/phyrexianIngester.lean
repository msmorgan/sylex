/-- Phyrexian Ingester -/
card phyrexianIngester : Spelled :=
  { name := "Phyrexian Ingester", cost := [generic 6, pip .blue], types := [.creature],
    subtypes := [phyrexian, beast],
    text :=
      [ imprint
          (when (does thisCreature enter)
            (may (exile (target (and [isCreature, isNontoken]))))),
        (static (conjunction [ getsPt thisCreature (up (letter .x)) (up (letter .y)),
                letterDefinition .x
                  (powerOf (the (and [isCreature, in_ (exiledWith thisCreature)]))),
                letterDefinition .y (toughnessOf (that card)) ])) ],
    power := stat 3, toughness := stat 3 }
