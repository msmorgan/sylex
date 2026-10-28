/-- Charnel Troll -/
card := {
  name := "Charnel Troll",
  cost := [1, black, green],
  types := [creature],
  subtypes := [troll],
  text := [
    trample,
    at_ (beginningOfYour upkeep)
      (doIfDone (exile (a [isCreature, in_ yourGraveyard])) (putCounters 1 p1p1Counter thisCreature)
         (sacrifice thisCreature)),
    activated (compound [[black, green], (discard (a [isCreature, in_ yourHand]))])
      (putCounters 1 p1p1Counter thisCreature)
  ],
  power := 4,
  toughness := 4
}
