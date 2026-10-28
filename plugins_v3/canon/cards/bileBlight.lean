/-- Bile Blight -/
card := {
  name := "Bile Blight",
  cost := [black, black],
  types := [instant],
  text := [
    spell
      (get
         (both (target isCreature)
            (allOf [isCreature, (shares cardName share (that creature)), isOther]))
         (down 3) (down 3) untilEndOfTurn)
  ]
}
