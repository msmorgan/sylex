card mindlockOrb : Spelled :=
  { name := "Mindlock Orb", cost := [generic 3, pip .blue], types := [.artifact],
    text := [static (cant (bare isPlayer) (search libraries))] }
