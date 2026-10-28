card := {
  name := "Void Mirror",
  cost := [2],
  types := [artifact],
  text := [
    whenever (does (a isPlayer) (cast (a isSpell))) (if_ := noColoredManaSpentToCast it)
      (counter (that spell))
  ]
}
