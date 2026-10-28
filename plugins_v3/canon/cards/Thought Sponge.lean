/-- Thought Sponge -/
card := {
  name := "Thought Sponge",
  cost := [3, blue],
  types := [creature],
  subtypes := [sponge],
  text := [
    flash,
    entersWithCounters thisCreature
      (aggregateOver max isOpponent (eventCount (draw anyAmount) they soFarThisTurn)) p1p1Counter,
    when (does thisCreature die) (draw (powerOf thisCreature))
  ],
  power := 1,
  toughness := 1
}
