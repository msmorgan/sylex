/-- Heat Wave -/
card heatWave : Spelled :=
  { name := "Heat Wave", cost := [generic 2, pip .red], types := [.enchantment],
    text :=
      [ cumulativeUpkeep [pip .red],
        static
            (cant (allOf (and [isCreature, hasColor .blue]))
               (block (allOf (and [isCreature, youControl])))),
        static
          (gatedBy (payLife (forEach 1 (and [isCreature, isBlocking, controlledBy they])))
            (actor (allOf (and [isCreature, non (hasColor .blue)])))
            (block (allOf (and [isCreature, youControl])))) ] }
