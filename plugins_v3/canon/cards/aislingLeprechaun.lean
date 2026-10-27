/-- Aisling Leprechaun -/
card aislingLeprechaun : Spelled :=
  { name := "Aisling Leprechaun", cost := [pip .green], types := [.creature],
    subtypes := [faerie],
    text :=
      [ whenever (does thisCreature (block (a isCreature)))
          (or_ := [does (by_ (a isCreature)) (block thisCreature)])
          (becomeColor (that creature) (.some [.green])) ],
    power := stat 1, toughness := stat 1 }
