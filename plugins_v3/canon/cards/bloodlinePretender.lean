/-- Bloodline Pretender -/
card := {
  name := "Bloodline Pretender",
  cost := [3],
  types := [artifact, creature],
  subtypes := [shapeshifter],
  text := [
    changeling,
    entersChoosing thisCreature (subtype creature),
    whenever (does (a [isCreature, youControl, isOther, ofChosen (subtype creature)]) enter)
      (putCounters 1 p1p1Counter thisCreature)
  ],
  power := 2,
  toughness := 2
}
