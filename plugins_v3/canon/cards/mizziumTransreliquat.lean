card mizziumTransreliquat : Spelled :=
  { name := "Mizzium Transreliquat", cost := [generic 3], types := [.artifact],
    text :=
      [ activated [generic 3]
          (establishThroughout (copyChange thisArtifact (target isArtifact)) untilEndOfTurn),
        activated [generic 1, pip .blue, pip .red]
          (establish (copyChange thisArtifact (target isArtifact) [exceptThisAbility])) ] }
