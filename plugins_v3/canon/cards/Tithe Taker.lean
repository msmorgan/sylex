card := {
  name := "Tithe Taker",
  cost := [1, white],
  types := [creature],
  subtypes := [human, soldier],
  text := [
    partScope turn you
      [
        (costShift (allOf [isSpell, did yourOpponents (cast which) thisObject]) (more 1)),
        costShift
          (allOf
             [
               abilityHead anyActivated,
               did yourOpponents (activate which) thisObject,
               not isManaAbility
             ])
          (more 1)
      ],
    afterlife 1
  ],
  power := 2,
  toughness := 1
}
