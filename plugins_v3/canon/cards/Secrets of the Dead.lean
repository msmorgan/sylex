card := {
  name := "Secrets of the Dead",
  cost := [2, blue],
  types := [enchantment],
  text := [
    whenever
      (does you (cast (a [isSpell, did any (cast which (from_ := yourGraveyard)) thisObject])))
      (draw 1)
  ]
}
