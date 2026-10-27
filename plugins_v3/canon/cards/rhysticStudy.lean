/-- Rhystic Study -/
card rhysticStudy : Spelled :=
  { name := "Rhystic Study", cost := [generic 2, pip .blue], types := [.enchantment],
    text :=
      [ whenever (does anOpponent (cast (a isSpell)))
          (instruct (that player) (doUnless (instruct you (may (draw 1)))
            [generic 1])) ] }
