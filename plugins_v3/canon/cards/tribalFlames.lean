/-- Tribal Flames -/
card tribalFlames : Spelled :=
  { name := "Tribal Flames", cost := [generic 1, pip .red], types := [.sorcery],
    text :=
      [ domain
          (spell (sequentially
            [ instruct this_ (dealDamage (letter .x) (target isAnyTarget)),
              define .x (distinctCount (.subtype .land .basicOnly)
                (allOf (and [isLand, youControl]))) ])) ] }
