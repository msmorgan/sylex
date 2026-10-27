/-- Danitha, New Benalia's Light -/
card danithaNewBenaliasLight : Spelled :=
  { name := "Danitha, New Benalia's Light", cost := [generic 1, pip .green, pip .white],
    supertypes := [.legendary], types := [.creature],
    subtypes := [human, knight],
    text :=
      [ vigilance, trample, lifelink,
        static (can you (cast (counted (exactly 1) (and [ isSpell, or [ aura,
                                  equipment ] ])) (from_ :=
                                                                yourGraveyard)) (timing :=
                                                                                   onEachOfYourTurns)) ],
    power := stat 2, toughness := stat 2 }
