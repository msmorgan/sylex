/-- Seedborn Muse -/
card := {
  name := "Seedborn Muse",
  cost := [3, green, green],
  types := [creature],
  subtypes := [spirit],
  text := [untapDuring (allOf youControl) (each isOtherPlayer)],
  power := 2,
  toughness := 4
}
