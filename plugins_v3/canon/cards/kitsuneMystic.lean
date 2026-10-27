card kitsuneMystic : Spelled := { body := .flip
  { characteristics :=
    { name := "Kitsune Mystic", cost := [generic 3, pip .white], types := [.creature],
      subtypes := [fox, wizard],
      text := [kitsuneMysticFlip], power := stat 2, toughness := stat 3 } }
  { characteristics :=
    { name := "Autumn-Tail, Kitsune Sage", supertypes := [.legendary], types := [.creature],
      subtypes := [fox, wizard],
      text :=
        [ activated [generic 1]
            (attachTo
              (target (and [aura, attachedTo (a isCreature)]))
              (a (and [isCreature, isOther]))) ],
      power := stat 4, toughness := stat 5 } } }
