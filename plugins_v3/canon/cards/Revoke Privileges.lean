card := {
  name := "Revoke Privileges",
  cost := [2, white],
  types := [enchantment],
  subtypes := [aura],
  text := [
    enchant isCreature,
    static
      [
        cant (enchanted creature) attack,
        cant (enchanted creature) block,
        cant (enchanted creature) (crew (allOf (hasSubtype vehicle)))
      ]
  ]
}
