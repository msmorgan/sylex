card waxWane : Spelled := { body := .split
  { characteristics :=
    { name := "Wax", cost := [pip .green], types := [.instant],
      text :=
        [ spell (get (target isCreature) (up 2) (up 2) untilEndOfTurn) ] } }
  { characteristics :=
    { name := "Wane", cost := [pip .white], types := [.instant],
      text := [spell (destroy (target isEnchantment))] } } }
