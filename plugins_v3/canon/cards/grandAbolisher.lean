/-- Grand Abolisher -/
card := {
  name := "Grand Abolisher",
  cost := [white, white],
  types := [creature],
  subtypes := [human, cleric],
  text := [
    partScope turn you
      [
        cant yourOpponents (cast (bare isSpell)),
        cant yourOpponents
          (activate
             (allOf
                [
                  abilityHead anyActivated,
                  abilityOf (allOf (or [isArtifact, isCreature, isEnchantment]))
                ]))
      ]
  ],
  power := 2,
  toughness := 2
}
