/-- Lieutenants of the Guard -/
card lieutenantsOfTheGuard : Spelled :=
  { name := "Lieutenants of the Guard", cost := [generic 4, pip .white],
    types := [.creature], subtypes := [human, soldier],
    text :=
      [ councilsDilemma
          (when (does thisCreature enter)
            (sequentially
              [ instruct eachPlayer (voteStartingWith you .openly
                  (byLabel ["strength", "numbers"])),
                putCounters (votesFor "strength") p1p1Counter thisCreature,
                create (votesFor "numbers") (creatureToken 1 1 [.white] [soldier]) ])) ],
    power := stat 2, toughness := stat 2 }
