/-- Starbreach Whale. Warp's "exile the permanent this spell becomes" is a Resolution Delayed
Trigger [CR#702.185a,608.3g]. -/
card := {
  name := "Starbreach Whale",
  cost := [4, blue],
  types := [creature],
  subtypes := [whale],
  text := [flying, when (does thisCreature enter) (surveil 2), warp [1, blue]],
  power := 3,
  toughness := 5
}
