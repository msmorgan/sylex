/-- Verity Circle -/
card verityCircle : Spelled :=
  { name := "Verity Circle", cost := [generic 2, pip .blue], types := [.enchantment],
    text :=
      [ whenever
          (does (a (and [isCreature, anOpponentControls])) (setStatus tapped theActor))
          (if_ := is it (not (inCombat .declaredAttacker)))
          (may (draw 1)),
        activated [generic 4, pip .blue]
          (tap (target (and [isCreature, not (hasKeyword flying)]))) ] }
