/-- Obeka, Splitter of Seconds -/
card obekaSplitterOfSeconds : Spelled :=
  { name := "Obeka, Splitter of Seconds",
    cost := [generic 1, pip .blue, pip .black, pip .red],
    supertypes := [.legendary], types := [.creature],
    subtypes := [ogre, warlock],
    text :=
      [ menace,
        whenever (does thisCreature (dealDamage (to := a isPlayer) (kind := some .combat)))
          (additionalStep .upkeep none thatMuch) ],
    power := stat 2, toughness := stat 5 }
