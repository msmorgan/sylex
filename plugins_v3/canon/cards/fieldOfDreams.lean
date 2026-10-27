/-- Field of Dreams -/
card fieldOfDreams : Spelled :=
  { name := "Field of Dreams", cost := [pip .blue], supertypes := [.world],
    types := [.enchantment],
    text := [static (visibility .reveal (bare isPlayer) topOfLibrary)] }
