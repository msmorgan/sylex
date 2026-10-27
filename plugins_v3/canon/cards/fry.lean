/-- Fry -/
card fry : Spelled :=
  { name := "Fry", cost := [generic 1, pip .red], types := [.instant],
    text :=
      [ static (cant any (counter thisSpell)),
        spell (instruct this_ (dealDamage 5
                 (target
                    (and
                       [or [isCreature, isPlaneswalker], or
                          [hasColor .white, hasColor .blue]])))) ] }
