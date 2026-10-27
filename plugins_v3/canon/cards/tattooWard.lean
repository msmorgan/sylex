/-- Tattoo Ward -/
card tattooWard : Spelled :=
  { name := "Tattoo Ward", cost := [generic 2, pip .white], types := [.enchantment],
    subtypes := [aura],
    text :=
      [ enchant isCreature,
        static (conjunction
          [ getsPt (enchanted creature) (up 1) (up 1),
            retention (abilityGrant (enchanted creature) (protection isEnchantment)) thisAura ]
          (subject := enchanted creature)),
        activated (sacrifice thisAura) (destroy (target isEnchantment)) ]
            }
