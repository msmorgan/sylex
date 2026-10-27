card ensnaringBridge : Spelled :=
  { name := "Ensnaring Bridge", cost := [generic 3], types := [.artifact],
    text :=
      [ static (cant (allOf (and [ isCreature,
                         hasPower .greater (countOf (cardIn yourHand)) ])) attack) ] }
