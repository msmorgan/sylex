template maritLage : CharacteristicBundle := {
  -- TODO(spelling): Preserve the exemplar phrasing.
  body := { characteristics :=
    { name := "Marit Lage", colors := [.black], supertypes := [.legendary], types := [.creature],
      subtypes := [avatar], text := [flying, indestructible],
      power := stat 20, toughness := stat 20 } }
}
