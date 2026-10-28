/-- An energy counter is a counter on a player. Paying {E} removes one energy counter from that
player. [CR#107.14] -/
counter energy := {
  lexical := { lemma := "energy counter" },
  spelling := "energy counter",
  holder := player,
  confers := []
}
