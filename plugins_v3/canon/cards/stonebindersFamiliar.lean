/-- Stonebinder's Familiar -/
card stonebindersFamiliar : Spelled :=
  { name := "Stonebinder's Familiar", cost := [pip .white], types := [.creature],
    subtypes := [spirit, dog],
    text :=
      [ whenever (does any (putInto (counted (orMore 1) isCard) exileZone))
          (during := duringPart .turn you)
          (putCounters 1 p1p1Counter thisCreature) (limit := oncePerTurn) ],
    power := stat 1, toughness := stat 1 }
