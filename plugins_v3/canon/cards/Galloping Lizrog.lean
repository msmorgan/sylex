card := {
  name := "Galloping Lizrog",
  cost := [3, green, blue],
  types := [creature],
  subtypes := [frog, lizard],
  text := [
    trample,
    when (does thisCreature enter)
      (may (removeCounters anyNumber p1p1Counter (among (allOf [isCreature, youControl])))
         (putCounters (times 2 thatMuch) p1p1Counter thisCreature))
  ],
  power := 3,
  toughness := 3
}
