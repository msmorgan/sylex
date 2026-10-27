/-- Scourge of Nel Toth -/
card scourgeOfNelToth : Spelled :=
  { name := "Scourge of Nel Toth", cost := [generic 5, pip .black, pip .black],
    types := [.creature], subtypes := [zombie, dragon],
    text :=
      [ flying,
        static (can you
                  (cast this_ (from_ := yourGraveyard) (payment := payingInstead (compound
              [ [pip .black, pip .black],
                (sacrifice (counted (exactly 2) isCreature)) ])))) ],
    power := stat 6, toughness := stat 6 }
