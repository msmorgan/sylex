/-- Goring Warplow -/
card goringWarplow : Spelled := { body := .prototype
  { characteristics :=
    { name := "Goring Warplow", cost := [generic 6], types := [.artifact, .creature],
      subtypes := [construct], text := [prototype, deathtouch],
      power := stat 5, toughness := stat 4 } }
  (prototypeAlt [generic 1, pip .black] 1 1) }
