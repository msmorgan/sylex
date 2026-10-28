card := {
  name := "Thranduil's Company",
  cost := [2, green, blue],
  types := [creature],
  subtypes := [elf, soldier],
  text := [
    onlyWhile (can you (play (counted (additional (exactly 1)) isLand)))
      (exists_ [[isCreature, isOther], elf, youControl]),
    landfall
      (whenever (does (a [isLand, youControl]) enter)
         [
           putCounters 2 p1p1Counter (target [isCreature, youControl]),
           gain it vigilance untilEndOfTurn
         ])
  ],
  power := 3,
  toughness := 4
}
