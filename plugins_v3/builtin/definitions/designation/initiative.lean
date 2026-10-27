/- Two of the initiative's three abilities venture into Undercity [CR#726.2,701.49d], and the
grammar has no venture action: venture into the dungeon [CR#701.49] needs a dungeon card frame
with its rooms and the venture marker's position [CR#309.4,309.5] (`lean-v3-venture-dungeon`).
The row carries the third until they can be written. -/

/-- The initiative: only one player can have it at a time [CR#726.3], so "the player who has the
initiative" names one player.

 [CR#726.1,726.2] The initiative: an effect makes a player take it, and the controller of
creatures that deal combat damage to the player who has it takes it. -/
designation initiative := {
  lexical := { nouns := ["initiative"] },
  spelling := "initiative",
  scope := (.heldBy .player),
  sole := true,
  confers := [initiativeSteal]
}
