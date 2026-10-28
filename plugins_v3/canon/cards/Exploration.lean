/-- Pins a continuous increase to the number of lands a Player may play during
their turn [CR#305.2]. -/
card := {
  name := "Exploration",
  cost := [green],
  types := [enchantment],
  text := [can you (play (counted (additional (exactly 1)) isLand)) (timing := onEachOfYourTurns)]
}
