/-- Energybending -/
card := {
  name := "Energybending",
  cost := [2],
  types := [instant],
  subtypes := [lesson],
  text := [
    spell
      [
        establishThroughout
          (qualityChange (allOf [isLand, youControl]) adds (everyTypeOf basicLand)) untilEndOfTurn,
        draw 1
      ]
  ]
}
