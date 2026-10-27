/-- Orchard Elemental -/
card orchardElemental : Spelled :=
  { name := "Orchard Elemental", cost := [generic 5, pip .green], types := [.creature],
    subtypes := [elemental],
    text :=
      [ councilsDilemma
          (when (does thisCreature enter)
            (sequentially
              [ instruct eachPlayer (voteStartingWith you .openly
                  (byLabel ["sprout", "harvest"])),
                putCounters (times 2 (votesFor "sprout")) p1p1Counter
                  thisCreature,
                gainLife (times 3 (votesFor "harvest")) ])) ],
    power := stat 2, toughness := stat 2 }
