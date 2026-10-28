card := {
  name := "Ichor Shade",
  cost := [2, black],
  types := [creature],
  subtypes := [phyrexian, shade],
  text := [
    at_ (beginningOfYour endStep)
      (if_ := some
         (was (putInto (a (or [isArtifact, isCreature])) graveyard (from_ := battlefield))
            soFarThisTurn))
      (putCounters 1 p1p1Counter thisCreature)
  ],
  power := 2,
  toughness := 3
}
