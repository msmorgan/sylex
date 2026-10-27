/-- Myr Prototype -/
card myrPrototype : Spelled :=
  { name := "Myr Prototype", cost := [generic 5], types := [.artifact, .creature],
    subtypes := [myr],
    text :=
      [ at_ (beginningOfYour .upkeep)
          (putCounters 1 p1p1Counter thisCreature),
        static
            (conjunction
               [gatedBy (scaledGenericMana (times 1 (countersOn p1p1Counter it)))
                  thisCreature attack, gatedBy
            (scaledGenericMana (times 1 (countersOn p1p1Counter it))) thisCreature
            block]) ],
    power := stat 2, toughness := stat 2 }
