card := {
  name := "Chrome Replicator",
  cost := [5],
  types := [artifact, creature],
  subtypes := [construct],
  text := [
    when (does thisCreature enter)
      (if_ := thereAre
         (withTheSameName (counted (orMore 2) [isPermanent, non isLand, isNontoken, youControl])))
      (create 1
         (written
            { types := [artifact, creature], subtypes := [construct], power := 4, toughness := 4 }))
  ],
  power := 4,
  toughness := 4
}
