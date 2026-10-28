/-- Containment Priest -/
card := {
  name := "Containment Priest",
  cost := [1, white],
  types := [creature],
  subtypes := [human, cleric],
  text := [
    flash,
    replace (would (a [isCreature, isNontoken, not (did any (cast which) thisObject)]) enter)
      (instead (exile it))
  ],
  power := 2,
  toughness := 2
}
