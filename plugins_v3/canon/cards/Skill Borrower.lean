card := {
  name := "Skill Borrower",
  cost := [2, blue],
  types := [artifact, creature],
  subtypes := [human, wizard],
  text := [
    visibility reveal you topOfLibrary,
    onlyWhile (abilityGrantFrom thisCreature (topSlice 1) [anyActivated])
      (is (topSlice 1) (or [isArtifact, isCreature]))
  ],
  power := 1,
  toughness := 3
}
