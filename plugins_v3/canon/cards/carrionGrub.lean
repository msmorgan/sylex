card := {
  name := "Carrion Grub",
  cost := [3, black],
  types := [creature],
  subtypes := [insect],
  text := [
    static
      [
        getsPt thisCreature (up (letter x)) (up 0),
        letterDefinition x (aggregate max (stat power) [isCreature, in_ yourGraveyard])
      ],
    when (does thisCreature enter) (mill 4)
  ],
  power := 0,
  toughness := 5
}
