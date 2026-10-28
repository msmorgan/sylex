card := {
  name := "Quick Sliver",
  cost := [1, green],
  types := [creature],
  subtypes := [sliver],
  text := [flash, can (a isPlayer) (cast (bare [isSpell, sliver])) (asThough := asThoughHadFlash them)],
  power := 1,
  toughness := 1
}
