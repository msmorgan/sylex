/-- Phyrexian Ingester -/
card phyrexianIngester : Spelled :=
  { name := "Phyrexian Ingester", cost := [generic 6, pip .blue], types := [.creature],
    subtypes := [phyrexian, beast],
    text :=
      [ imprint
          (when (does thisCreature enter)
            (may (exile (target (and [isCreature, isNontoken]))))),
        phyrexianIngesterPump ],
    power := stat 3, toughness := stat 3 }
