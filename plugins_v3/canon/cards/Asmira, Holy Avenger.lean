card := {
  name := "Asmira, Holy Avenger",
  cost := [2, green, white],
  supertypes := [legendary],
  types := [creature],
  subtypes := [human, cleric],
  text := [
    flying,
    at_ (beginningOfPossessed endStep eachPlayer)
      (putCounters
         (eventCount (putInto (a isCreature) yourGraveyard (from_ := battlefield)) any soFarThisTurn)
         p1p1Counter thisCreature)
  ],
  power := 2,
  toughness := 3
}
