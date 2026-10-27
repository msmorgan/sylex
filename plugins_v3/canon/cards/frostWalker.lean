/-- Frost Walker -/
card frostWalker : Spelled :=
  { name := "Frost Walker", cost := [generic 1, pip .blue], types := [.creature],
    subtypes := [elemental],
    text :=
      [ when (does (a (or [isSpell, abilityHead .anyOnStack])) (target thisCreature))
          (sacrifice it) ],
    power := stat 4, toughness := stat 1 }
