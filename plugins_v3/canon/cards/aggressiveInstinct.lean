/-- Aggressive Instinct -/
card aggressiveInstinct : Spelled :=
  { name := "Aggressive Instinct", cost := [generic 1, pip .green], types := [.sorcery],
    text :=
      [ spell (dealDamageOwnPower (target (and [isCreature, youControl]))
          (target (and [isCreature, youDontControl]))) ] }
