card := {
  name := "Ichor Shade",
  cost := [2, black],
  types := [creature],
  subtypes := [phyrexian, shade],
  text := [
    at_ (beginningOfYour endStep)
      (if_ := some
         (was (putIntoFromBattlefield (a (or [isArtifact, isCreature])) graveyard)
            soFarThisTurn))
      (putCounters 1 p1p1Counter thisCreature)
  ],
  power := 2,
  toughness := 3
}
