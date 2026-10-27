card frogify : Spelled :=
  { name := "Frogify", cost := [generic 1, pip .blue], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isCreature,
        static ((conjunction [ allAbilityLoss (enchanted creature),
                qualityChange it .sets
                  (bundle { characteristics := { colors := [.blue], types := [.creature], subtypes
                    := [frog] } }),
                getsBase it (amountLit 1) (amountLit 1) ])) ] }
