card := {
  name := "Aladdin's Ring",
  cost := [8],
  types := [artifact],
  text := [
    activated (compound [[8], tapSymbol])
      (instruct thisArtifact (dealDamage 4 (target isAnyTarget)))
  ]
}
