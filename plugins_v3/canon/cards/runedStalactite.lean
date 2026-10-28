card := {
  name := "Runed Stalactite",
  cost := [1],
  types := [artifact],
  subtypes := [equipment],
  text := [
    static
      [
        getsPt (equipped creature) (up 1) (up 1),
        qualityChange (equipped creature) adds (everyTypeOf creature)
      ],
    equip [2]
  ]
}
