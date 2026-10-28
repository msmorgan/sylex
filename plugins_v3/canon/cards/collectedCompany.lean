/-- Collected Company -/
card := {
  name := "Collected Company",
  cost := [3, green],
  types := [instant],
  text := [
    spell
      [
        lookAt (topSlice 6),
        putOntoBattlefield (fromAmong (upTo 2) [isCreature, hasManaValue atMost 3] them),
        putInto theRest onBottom (riders := [(inOrder anyOrder)])
      ]
  ]
}
