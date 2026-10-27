card branchingEvolution : Spelled :=
  { name := "Branching Evolution", cost := [generic 2, pip .green], types := [.enchantment],
    text :=
      [ static
          (replace
             (would any
                (putCounters anyAmount p1p1Counter
                   (a (and [isCreature, youControl]))))
          (instead (putCounters (times 2 thatMuch) p1p1Counter (that creature)))) ] }
