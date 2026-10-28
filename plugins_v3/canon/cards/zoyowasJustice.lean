card := {
  name := "Zoyowa's Justice",
  cost := [1, red],
  types := [instant],
  text := [
    spell
      [
        instruct (ownerOf (target [or [isArtifact, isCreature], hasManaValue atLeast 1]))
          (shuffleInto it),
        instruct (that player) (discover (statOf (stat manaValue) it))
      ]
  ]
}
