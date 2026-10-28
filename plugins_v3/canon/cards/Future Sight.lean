card := {
  name := "Future Sight",
  cost := [2, blue, blue, blue],
  types := [enchantment],
  text := [
    visibility reveal you topOfLibrary,
    static
      [
        (can you (play (allOf isLand) (from_ := onTop))),
        (can you (cast (bare isSpell) (from_ := onTop)))
      ]
  ]
}
