card := {
  name := "Deadeye Brawler",
  cost := [2, blue, black],
  types := [creature],
  subtypes := [human, pirate],
  text := [
    deathtouch,
    ascend,
    whenever (does thisCreature (dealDamage (to := a isPlayer) (kind := some combat)))
      (if_ := is you (hasDesignation citysBlessing)) (draw 1)
  ],
  power := 2,
  toughness := 4
}
