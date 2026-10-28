/-- Stonebinder's Familiar -/
card := {
  name := "Stonebinder's Familiar",
  cost := [white],
  types := [creature],
  subtypes := [spirit, dog],
  text := [
    whenever (does any (putInto (counted (orMore 1) isCard) exileZone))
      (during := duringPart turn you) (putCounters 1 p1p1Counter thisCreature)
      (limit := oncePerTurn)
  ],
  power := 1,
  toughness := 1
}
