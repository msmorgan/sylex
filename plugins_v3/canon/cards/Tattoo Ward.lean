card := {
  name := "Tattoo Ward",
  cost := [2, white],
  types := [enchantment],
  subtypes := [aura],
  text := [
    enchant isCreature,
    conjunction
      [
        getsPt (enchanted creature) (up 1) (up 1),
        retention (abilityGrant (enchanted creature) (protection isEnchantment)) thisAura
      ]
      (subject := enchanted creature),
    activated (sacrifice thisAura) (destroy (target isEnchantment))
  ]
}
