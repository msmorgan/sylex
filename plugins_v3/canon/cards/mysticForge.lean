card := {
  name := "Mystic Forge",
  cost := [4],
  types := [artifact],
  text := [
    visibility lookAt you topOfLibrary,
    static
      [
        (can you (cast (bare [isSpell, isArtifact]) (from_ := onTop))),
        (can you (cast (bare [isSpell, isColorless]) (from_ := onTop)))
      ],
    activated (compound [tapSymbol, payLife 1]) (exile (topSlice 1))
  ]
}
