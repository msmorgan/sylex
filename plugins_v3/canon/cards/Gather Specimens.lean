card := {
  name := "Gather Specimens",
  cost := [3, blue, blue, blue],
  types := [instant],
  text := [
    spell
      (establishThroughout
         (replace (would (a isCreature) (enter (riders := [under anOpponent])))
            (instead (instruct it (enter (riders := [under you])))))
         thisTurn)
  ]
}
