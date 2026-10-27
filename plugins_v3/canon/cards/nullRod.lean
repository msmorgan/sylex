card nullRod : Spelled :=
  { name := "Null Rod", cost := [generic 2], types := [.artifact],
    text := [static ((cant any (activate (allOf (and [abilityHead .anyActivated, abilityOf (allOf
      isArtifact)])))))] }
