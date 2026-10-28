card := {
  name := "Suppression Field",
  cost := [1, white],
  types := [enchantment],
  text := [costShift (allOf [abilityHead anyActivated, not isManaAbility]) (more 2)]
}
