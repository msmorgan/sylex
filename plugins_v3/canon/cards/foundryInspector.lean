card := {
  name := "Foundry Inspector",
  cost := [3],
  types := [artifact, creature],
  subtypes := [construct],
  text := [costShift (allOf [isArtifact, isSpell, did you (cast which) thisObject]) (less 1)],
  power := 3,
  toughness := 2
}
