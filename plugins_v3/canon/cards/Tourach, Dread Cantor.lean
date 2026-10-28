/-- Tourach, Dread Cantor -/
card := {
  name := "Tourach, Dread Cantor",
  cost := [1, black],
  supertypes := [legendary],
  types := [creature],
  subtypes := [human, cleric],
  text := [
    kicker [black, black],
    protection (hasColor white),
    (whenever (does anOpponent (discard (a isCard))) (putCounters 1 p1p1Counter thisCreature)),
    when (does thisCreature enter) (if_ := keywordCostPaid kicker thisCreature)
      (instruct (target isOpponent) (discard (countedAtRandom (exactly 2) isCard)))
  ],
  power := 2,
  toughness := 1
}
