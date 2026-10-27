/-- Retro-Mutation -/
card retroMutation : Spelled :=
  { name := "Retro-Mutation", cost := [generic 2, pip .blue], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ flash,
        enchant isCreature,
        static (conjunction [ qualityChange (enchanted creature) .sets
              (bundle { characteristics := { subtypes := [turtle] } }),
            getsBase it 0 1,
            cant it attack,
            allAbilityLoss it ]) ] }
