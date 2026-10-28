/-- [CR#122.1] "A counter is a marker placed on an object or player." A charge counter is a
marker and nothing else: the CR gives it no behavior of its own, so it confers nothing. -/
counter chargeCounter := {
  lexical := { lemma := "charge counter" },
  spelling := "charge counter",
  holder := object,
  confers := []
}
