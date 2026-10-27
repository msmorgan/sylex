card mysticForge : Spelled :=
  { name := "Mystic Forge", cost := [generic 4], types := [.artifact],
    text :=
      [ static (visibility .lookAt you topOfLibrary),
        static (conjunction [castFromTop (and [isSpell, isArtifact]), castFromTop (and
            [isSpell, isColorless])]),
        activated (compound [tapSymbol, payLife 1]) (exile (topSlice 1)) ] }
