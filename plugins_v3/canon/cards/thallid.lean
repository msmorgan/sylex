/-- Thallid -/
card thallid : Spelled :=
  { name := "Thallid", cost := [pip .green], types := [.creature],
    subtypes := [fungus],
    text :=
      [ at_ (beginningOfYour .upkeep)
          (putCounters 1 sporeCounter thisCreature),
        activated
          (removeCounters (exactly 3) sporeCounter thisCreature)
          (create 1 (creatureToken 1 1 [.green] [saproling])) ],
    power := stat 1, toughness := stat 1 }
