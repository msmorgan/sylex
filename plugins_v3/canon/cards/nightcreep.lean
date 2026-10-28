/-- Nightcreep -/
card := {
  name := "Nightcreep",
  cost := [black, black],
  types := [instant],
  text := [
    spell
      (establishThroughout
         [
           qualityChange (allOf isCreature) sets (colored (.some [black])),
           qualityChange (allOf isLand) sets (bundle { subtypes := [swamp] })
         ]
         untilEndOfTurn)
  ]
}
