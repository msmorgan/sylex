/-- Frogmite -/
card frogmite : Spelled :=
  { name := "Frogmite", cost := [generic 4], types := [.artifact, .creature],
    subtypes := [frog], text := [affinity isArtifact],
    power := stat 2, toughness := stat 2 }
