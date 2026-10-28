card := {
  name := "Amorphous Axe",
  cost := [2],
  types := [artifact],
  subtypes := [equipment],
  text := [
    static
      [
        getsPt (equipped creature) (up 3) (up 0),
        qualityChange (equipped creature) adds (everyTypeOf creature)
      ],
    equip [3]
  ]
}
