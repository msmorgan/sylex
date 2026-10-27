card mysticForge : Spelled :=
  { name := "Mystic Forge", cost := [generic 4], types := [.artifact],
    text :=
      [ static (visibility .lookAt you topOfLibrary),
        static (conjunction [(can you (cast (bare (and [isSpell, isArtifact])) (from_ := onTop))),
          (can you (cast (bare (and
                      [isSpell, isColorless])) (from_ := onTop)))]),
        activated (compound [tapSymbol, payLife 1]) (exile (topSlice 1)) ] }
