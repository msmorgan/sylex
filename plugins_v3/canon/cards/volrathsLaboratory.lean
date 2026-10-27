/-- Volrath's Laboratory -/
card volrathsLaboratory : Spelled :=
  { name := "Volrath's Laboratory", cost := [generic 5], types := [.artifact],
    text :=
      [ static volrathsLaboratoryChoice,
        activated (compound [[generic 5], tapSymbol])
          (create 1 (written
            { characteristics := { types := [.creature], power := stat 2, toughness := stat 2 },
              qualities := [withQuality (ofChosen .color), withQuality (ofChosen (.subtype .creature))] })) ] }
