/-- Vexing Shusher -/
card vexingShusher : Spelled :=
  { name := "Vexing Shusher", cost := [hybridPip .red .green, hybridPip .red .green],
    types := [.creature], subtypes := [goblin, shaman],
    text :=
      [ static (cant any (counter thisSpell)),
        activated [hybridPip .red .green]
          (establish (cant any (counter (target isSpell)))) ],
    power := stat 2, toughness := stat 2 }
