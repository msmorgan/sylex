/-- Gideon's Intervention -/
card := {
  name := "Gideon's Intervention",
  cost := [2, white, white],
  types := [enchantment],
  text := [
    entersChoosing thisEnchantment cardName,
    cant yourOpponents (cast (bare [isSpell, named chosenName])),
    replace
      (would (by_ (allOf [isSource, named chosenName]))
         (dealDamage anyAmount (youAnd (allOf [isPermanent, youControl]))))
      (instead (preventDamage allDamage))
  ]
}
