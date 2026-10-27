card spellSnare : Spelled :=
  { name := "Spell Snare", cost := [pip .blue], types := [.instant],
    text :=
      [ spell (counter (target (and [isSpell, hasManaValue .eq 2]))) ] }
