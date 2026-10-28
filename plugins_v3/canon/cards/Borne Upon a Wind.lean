card := {
  name := "Borne Upon a Wind",
  cost := [1, blue],
  types := [instant],
  text := [
    spell
      (establishThroughout (can you (cast (bare isSpell)) (asThough := asThoughHadFlash them))
         thisTurn),
    spell (draw 1)
  ]
}
