card := {
  name := "Endure",
  cost := [3, white, white],
  types := [instant],
  text := [spell (preventAll (youAnd (allOf [isPermanent, youControl])) thisTurn)]
}
