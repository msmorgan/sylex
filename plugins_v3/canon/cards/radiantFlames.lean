/-- Radiant Flames (converge is an ability word [CR#207.2c]) -/
card radiantFlames : Spelled :=
  { name := "Radiant Flames", cost := [generic 2, pip .red], types := [.sorcery],
    text := [spell (instruct this_ (dealDamage (colorsSpentToCast this_) (each isCreature)))] }
