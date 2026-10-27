card amphibianDownpour : Spelled :=
  { name := "Amphibian Downpour", cost := [generic 2, pip .blue], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ flash, storm,
        enchant isCreature,
        static ((conjunction [ allAbilityLoss (enchanted creature),
                qualityChange it .sets
                  (bundle { characteristics := { colors := [.blue], types := [.creature], subtypes
                    := [frog] } }),
                getsBase it (amountLit 1) (amountLit 1) ])) ] }
