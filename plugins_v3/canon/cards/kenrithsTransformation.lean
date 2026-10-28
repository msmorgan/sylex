card kenrithsTransformation : Spelled :=
  { name := "Kenrith's Transformation", cost := [generic 1, pip .green],
    types := [.enchantment], subtypes := [aura],
    text :=
      [ enchant isCreature,
        when (does thisAura enter) (draw 1),
        static ((conjunction [ allAbilityLoss (enchanted creature),
                qualityChange it .sets
                  (bundle { characteristics := { colors := [.green], types := [.creature], subtypes
                    := [elk] } }),
                getsBase it (lit 3) (lit 3) ])) ] }
