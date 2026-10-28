/-- Leyline of Vitality -/
card := {
  name := "Leyline of Vitality",
  cost := [2, green, green],
  types := [enchantment],
  text := [
    mayBeginOnBattlefield,
    getsPt (allOf [isCreature, youControl]) (up 0) (up 1),
    whenever (does (a [isCreature, youControl]) enter) (may (gainLife 1))
  ]
}
