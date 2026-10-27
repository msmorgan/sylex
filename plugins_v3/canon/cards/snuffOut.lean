card snuffOut : Spelled :=
  { name := "Snuff Out", cost := [generic 3, pip .black], types := [.instant],
    text :=
      [ static (onlyWhile (altCost this_ (payLife 4))
          (exists_ (and [isLand, swamp, youControl]))),
        spell (destroyCantRegenerate (target (and [isCreature, non (hasColor .black)]))) ] }
