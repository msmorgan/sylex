card := {
  name := "Magus of the Future",
  cost := [2, blue, blue, blue],
  types := [creature],
  subtypes := [human, wizard],
  text := [
    visibility reveal you topOfLibrary,
    static
      [
        (can you (play (allOf isLand) (from_ := onTop))),
        (can you (cast (bare isSpell) (from_ := onTop)))
      ]
  ],
  power := 2,
  toughness := 3
}
