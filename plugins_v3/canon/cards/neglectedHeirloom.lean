/-- Neglected Heirloom // Ashmouth Blade -/
card := transforming
  {
    front := {
      name := "Neglected Heirloom",
      cost := [1],
      types := [artifact],
      subtypes := [equipment],
      text := [
        getsPt (equipped creature) (up 1) (up 1),
        when (does any (turnOver (equipped creature))) (transform thisEquipment),
        equip [1]
      ]
    },
    back := {
      name := "Ashmouth Blade",
      types := [artifact],
      subtypes := [equipment],
      text := [static [getsPt (equipped creature) (up 3) (up 3), abilityGrant it firstStrike], equip [3]]
    }
  }
