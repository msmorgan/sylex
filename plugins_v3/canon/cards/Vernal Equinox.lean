card := {
  name := "Vernal Equinox",
  cost := [3, green],
  types := [enchantment],
  text := [
    can (a isPlayer) (cast (bare [isSpell, or [isCreature, isEnchantment]]))
      (asThough := asThoughHadFlash them)
  ]
}
