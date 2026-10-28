card := {
  name := "Precognition Field",
  cost := [3, blue],
  types := [enchantment],
  text := [
    visibility lookAt you topOfLibrary,
    can you (cast (bare [isSpell, isInstantOrSorcery]) (from_ := onTop)),
    activated [3] (exile (topSlice 1))
  ]
}
