/-- Blitz Automaton -/
card blitzAutomaton : Spelled := { body := .prototype
  { characteristics :=
    { name := "Blitz Automaton", cost := [generic 7], types := [.artifact, .creature],
      subtypes := [construct], text := [prototype, haste],
      power := stat 6, toughness := stat 4 } }
  (prototypeAlt [generic 2, pip .red] 3 2) }
