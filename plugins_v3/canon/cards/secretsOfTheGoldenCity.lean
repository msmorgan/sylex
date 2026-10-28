card := {
  name := "Secrets of the Golden City",
  cost := [1, blue, blue],
  types := [sorcery],
  text := [
    ascend,
    spell (replace (draw 2) (doIf (is you (hasDesignation citysBlessing)) (instead (draw 3))))
  ]
}
