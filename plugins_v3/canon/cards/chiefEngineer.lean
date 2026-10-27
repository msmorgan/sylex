card chiefEngineer : Spelled :=
  { name := "Chief Engineer", cost := [generic 1, pip .blue], types := [.creature],
    subtypes := [vedalken, artificer],
    text := [static (abilityGrant (allOf (and [isArtifact, isSpell,
        did you (cast which) thisObject])) convoke)],
    power := stat 1, toughness := stat 3 }
