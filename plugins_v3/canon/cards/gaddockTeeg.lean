/-- Gaddock Teeg -/
card gaddockTeeg : Spelled :=
  { name := "Gaddock Teeg", cost := [pip .green, pip .white], supertypes := [.legendary],
    types := [.creature], subtypes := [kithkin, advisor],
    text :=
      [ static
          (cant any
             (cast (bare (and [isSpell, non isCreature, hasManaValue .atLeast 4])))),
        static (cant any (cast (bare (and [isSpell, non isCreature, manaCostHas .variable])))) ],
    power := stat 2, toughness := stat 2 }
