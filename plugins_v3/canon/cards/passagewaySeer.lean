card passagewaySeer : Spelled :=
  { name := "Passageway Seer", cost := [generic 3, pip .black], types := [.creature],
    subtypes := [tiefling, warlock],
    text :=
      [ lifelink,
        when (does thisCreature enter) (gainDesignation you initiative),
        at_ (beginningOfYour .endStep)
          (if_ := is you (hasDesignation initiative))
          (putCounters 1 p1p1Counter thisCreature) ],
    power := stat 2, toughness := stat 2 }
