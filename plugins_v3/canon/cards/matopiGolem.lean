card matopiGolem : Spelled :=
  { name := "Matopi Golem", cost := [generic 5], types := [.artifact, .creature],
    subtypes := [golem],
    text :=
      [ activated [generic 1]
          (triggerThisWay (regenerate thisCreature) (does any (regeneration thisCreature))
            (putCounters 1 m1m1Counter it)) ],
    power := stat 3, toughness := stat 3 }
