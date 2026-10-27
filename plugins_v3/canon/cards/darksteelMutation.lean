card darksteelMutation : Spelled :=
  { name := "Darksteel Mutation", cost := [generic 1, pip .white], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isCreature,
        static (conjunction [ qualityChange (enchanted creature) .sets
              (bundle
                 { characteristics := { types := [.artifact, .creature], subtypes := [insect] }
                     }),
            getsBase it 0 1,
            abilityGrant it indestructible,
            allAbilityLoss it ]) ] }
