/-- Memory Plunder -/
card memoryPlunder : Spelled :=
  { name := "Memory Plunder",
    cost := [hybridPip .blue .black, hybridPip .blue .black, hybridPip .blue .black,
                  hybridPip .blue .black],
    types := [.instant],
    text :=
      [ spell
          (may
             (cast (target (and [isInstantOrSorcery, isCard]))
                (from_ := graveyardOf anOpponent) (payment := withoutPaying))) ] }
