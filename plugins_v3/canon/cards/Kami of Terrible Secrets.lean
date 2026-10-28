card := {
  name := "Kami of Terrible Secrets",
  cost := [3, black],
  types := [creature],
  subtypes := [spirit],
  text := [
    when (does thisCreature enter)
      (if_ := some [exists_ [isArtifact, youControl], exists_ [isEnchantment, youControl]])
      [draw 1, gainLife 1]
  ],
  power := 3,
  toughness := 4
}
