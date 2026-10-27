/-- Eager Construct -/
card eagerConstruct : Spelled :=
  { name := "Eager Construct", cost := [generic 2], types := [.artifact, .creature],
    subtypes := [construct],
    text := [when (does thisCreature enter) (instruct eachPlayer (may (scry 1)))],
    power := stat 2, toughness := stat 2 }
