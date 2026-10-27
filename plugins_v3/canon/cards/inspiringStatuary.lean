card inspiringStatuary : Spelled :=
  { name := "Inspiring Statuary", cost := [generic 3], types := [.artifact],
    text :=
      [ static (abilityGrant (allOf (and [non isArtifact, isSpell,
          did you (cast which) thisObject])) improvise) ] }
