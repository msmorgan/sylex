card := {
  name := "Mistbreath Elder",
  cost := [green],
  types := [creature],
  subtypes := [frog, warrior],
  text := [
    at_ (beginningOfYour upkeep)
      (doIfDone (returnToHand (a [isCreature, youControl, isOther]))
         (putCounters 1 p1p1Counter thisCreature) (may (returnToHand thisCreature)))
  ],
  power := 2,
  toughness := 2
}
