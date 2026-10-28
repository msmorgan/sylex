card := {
  name := "Hallowed Moonlight",
  cost := [1, white],
  types := [instant],
  text := [
    spell
      [
        replaceEvent (would (a [isCreature, not (did any (cast which) thisObject)]) enter)
          (instead (exile it)) untilEndOfTurn,
        draw 1
      ]
  ]
}
