/-- Ghostly Keybearer -/
card ghostlyKeybearer : Spelled :=
  { name := "Ghostly Keybearer", cost := [generic 3, pip .blue], types := [.creature],
    subtypes := [spirit],
    text :=
      [ flying,
        whenever (does thisCreature (dealDamage (to := a isPlayer) (kind := some .combat)))
          (unlock (doorOf (some .locked)
            (targets (upTo 1) (and [room, youControl])))) ],
    power := stat 3, toughness := stat 3 }
