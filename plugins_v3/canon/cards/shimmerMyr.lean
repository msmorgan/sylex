card shimmerMyr : Spelled :=
  { name := "Shimmer Myr", cost := [generic 3], types := [.artifact, .creature],
    subtypes := [myr],
    text :=
      [ flash,
        static (can you (cast (bare (and [isSpell, isArtifact]))) (asThough := asThoughHadFlash them)) ],
    power := stat 2, toughness := stat 2 }
