/-- Indigo Faerie -/
card := {
  name := "Indigo Faerie",
  cost := [1, blue],
  types := [creature],
  subtypes := [faerie, wizard],
  text := [
    flying,
    activated [blue]
      (establishThroughout (qualityChange (target isPermanent) adds (colored (.some [blue])))
         untilEndOfTurn)
  ],
  power := 1,
  toughness := 1
}
