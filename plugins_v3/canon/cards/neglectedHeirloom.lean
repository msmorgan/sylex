/-- Neglected Heirloom // Ashmouth Blade -/
card neglectedHeirloom : Spelled := { body := .transforming
  { characteristics :=
    { name := "Neglected Heirloom", cost := [generic 1], types := [.artifact],
      subtypes := [equipment],
      text :=
        [ static (getsPt (equipped creature) (up 1) (up 1)),
          when
            (does any (turnOver (equipped creature)))
            (transform thisEquipment),
          equip [generic 1] ] } }
  { characteristics :=
    { name := "Ashmouth Blade", types := [.artifact], subtypes := [equipment],
      text :=
        [ static (conjunction [ getsPt (equipped creature) (up 3) (up 3),
              abilityGrant it firstStrike ]),
          equip [generic 3] ] } } }
