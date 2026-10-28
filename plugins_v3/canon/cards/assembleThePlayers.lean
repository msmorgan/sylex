card := {
  name := "Assemble the Players",
  cost := [1, white],
  types := [enchantment],
  text := [
    visibility lookAt you topOfLibrary,
    can you (cast (counted (exactly 1) [isSpell, isCreature, hasPower atMost 2]) (from_ := onTop))
      (timing := eachTurn)
  ]
}
