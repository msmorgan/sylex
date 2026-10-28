card := {
  name := "Tromell, Seymour's Butler",
  cost := [2, green],
  supertypes := [legendary],
  types := [creature],
  subtypes := [elf, advisor],
  text := [
    entersWithAdditionalCounters (each [isCreature, isNontoken, youControl, isOther]) 1 p1p1Counter,
    activated (compound [[1], tapSymbol])
      [
        repeatTimes (letter x) proliferate,
        define x
          (countOf [isNontoken, isCreature, youControl, did which enter (within soFarThisTurn)])
      ]
  ],
  power := 2,
  toughness := 3
}
