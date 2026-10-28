card := {
  name := "Vega, the Watcher",
  cost := [1, white, blue],
  supertypes := [legendary],
  types := [creature],
  subtypes := [bird, spirit],
  text := [
    flying,
    whenever
      (does you (cast (a [isSpell, not (did any (cast which (from_ := yourHand)) thisObject)])))
      (draw 1)
  ],
  power := 2,
  toughness := 2
}
