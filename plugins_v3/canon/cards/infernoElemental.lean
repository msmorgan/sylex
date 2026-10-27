/-- Inferno Elemental -/
card infernoElemental : Spelled :=
  { name := "Inferno Elemental", cost := [generic 4, pip .red, pip .red], types := [.creature],
    subtypes := [elemental],
    text :=
      [ whenever (does thisCreature (block (a isCreature)))
          (or_ := [does (by_ (a isCreature)) (block thisCreature)])
          (instruct thisCreature (dealDamage 3 (that creature))) ],
    power := stat 4, toughness := stat 4 }
