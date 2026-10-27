/-- Containment Priest -/
card containmentPriest : Spelled :=
  { name := "Containment Priest", cost := [generic 1, pip .white], types := [.creature],
    subtypes := [human, cleric],
    text :=
      [ flash,
        static (replace
                  (would (a (and [isCreature, isNontoken,
                      not (did any (cast which) thisObject)])) enter)
                  (instead (exile it))) ],
    power := stat 2, toughness := stat 2 }
