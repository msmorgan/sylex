card coldStorage : Spelled :=
  { name := "Cold Storage", cost := [generic 4], types := [.artifact],
    text :=
      [ activated [generic 3] (exile (target (and [isCreature, youControl]))),
        activated (sacrifice thisArtifact)
          (returnToBattlefield (each (and [isCreature, cardIn (exiledWith thisArtifact)]))
            [(under you)]) ] }
