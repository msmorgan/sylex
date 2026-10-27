/-- Prismari Pianist -/
card prismariPianist : Spelled :=
  { name := "Prismari Pianist", cost := [generic 1, pip .red, pip .red], types := [.creature],
    subtypes := [djinn, bard],
    text :=
      [ whenever (does you (cast (a
        (and [isInstantOrSorcery, isSpell]))))
          (replace
            (create 1 (creatureToken 1 1 [.blue, .red] [elemental]))
            (doIf (compareAmt (statOf (.stat .manaValue) (that .spell)) .atLeast 5)
              (instead (create 3 asThose [])))) ],
    power := stat 2, toughness := stat 1 }
