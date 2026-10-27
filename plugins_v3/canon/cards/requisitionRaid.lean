/-- Requisition Raid -/
card requisitionRaid : Spelled :=
  { name := "Requisition Raid", cost := [pip .white], types := [.sorcery],
    text :=
      [ spell (chooseSpree
          [ ([generic 1], destroy (target isArtifact)),
            ([generic 1], destroy (target isEnchantment)),
            ([generic 1],
             putCounters 1 p1p1Counter
               (each (and [isCreature, controlledBy (target isPlayer)]))) ]) ] }
