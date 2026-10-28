card := {
  name := "Passageway Seer",
  cost := [3, black],
  types := [creature],
  subtypes := [tiefling, warlock],
  text := [
    lifelink,
    when (does thisCreature enter) (gainDesignation you initiative),
    at_ (beginningOfYour endStep) (if_ := is you (hasDesignation initiative))
      (putCounters 1 p1p1Counter thisCreature)
  ],
  power := 2,
  toughness := 2
}
