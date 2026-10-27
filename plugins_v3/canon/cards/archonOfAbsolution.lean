/-- Archon of Absolution -/
card archonOfAbsolution : Spelled :=
  { name := "Archon of Absolution", cost := [generic 3, pip .white], types := [.creature],
    subtypes := [archon],
    text :=
      [ flying,
        protection (hasColor .white),
        static (gatedBy (scaledGenericMana (times 1 groupSize)) (allOf isCreature)
                  (attack (youOr (allOf (and [isPlaneswalker, youControl]))))) ],
    power := stat 3, toughness := stat 2 }
