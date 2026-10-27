card foundryInspector : Spelled :=
  { name := "Foundry Inspector", cost := [generic 3], types := [.artifact, .creature],
    subtypes := [construct],
    text := [static (spellsCost (and [isArtifact, isSpell, did you (cast which) thisObject])
        (less 1))],
    power := stat 3, toughness := stat 2 }
