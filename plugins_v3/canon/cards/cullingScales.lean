card cullingScales : Spelled :=
  { name := "Culling Scales", cost := [generic 3], types := [.artifact],
    text :=
      [ at_ (beginningOfYour .upkeep)
          (destroy (target (and [isPermanent, non isLand,
            superlative .min (.stat .manaValue) (and [isPermanent, non isLand])]))) ] }
