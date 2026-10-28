card := {
  name := "Vault 75: Middle School",
  cost := [2, white, white],
  types := [enchantment],
  subtypes := [saga],
  text := [
    when (chapterMark [1]) (exile (allOf [isCreature, hasPower atLeast 4])),
    when (chapterMark [2, 3]) (putCounters 1 p1p1Counter (each [isCreature, youControl]))
  ]
}
