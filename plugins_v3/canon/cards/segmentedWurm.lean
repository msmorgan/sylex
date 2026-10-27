/-- Segmented Wurm: "Whenever this creature becomes the target of a spell or ability, put a
−1/−1 counter on it." -/
card segmentedWurm : Spelled :=
  { name := "Segmented Wurm", cost := [generic 3, pip .red, pip .green], types := [.creature],
    subtypes := [wurm],
    text :=
      [ whenever (does (a (or [isSpell, abilityHead .anyOnStack])) (target thisCreature))
          (putCounters 1 m1m1Counter it) ],
    power := stat 5, toughness := stat 5 }
