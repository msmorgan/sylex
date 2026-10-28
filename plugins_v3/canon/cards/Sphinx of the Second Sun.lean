/-- Sphinx of the Second Sun -/
card := {
  name := "Sphinx of the Second Sun",
  cost := [6, blue, blue],
  types := [creature],
  subtypes := [sphinx],
  text := [
    flying,
    at_ (beginningOfYour postcombatMain) (additionalPhase beginningPhase (some postcombatMain) 1)
  ],
  power := 6,
  toughness := 6
}
