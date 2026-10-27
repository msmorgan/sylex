/-- Sculpting Steel -/
card sculptingSteel : Spelled :=
  { name := "Sculpting Steel", cost := [generic 3], types := [.artifact],
    text := [static (enters thisArtifact [asCopyOf true (a isArtifact) []])] }
