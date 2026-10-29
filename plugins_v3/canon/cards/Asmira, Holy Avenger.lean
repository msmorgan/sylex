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
         (eventCount (putIntoFromBattlefield (a isCreature) yourGraveyard) any soFarThisTurn)
         p1p1Counter thisCreature)
  ],
  power := 2,
  toughness := 3
}
