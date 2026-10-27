card oonasBlackguard : Spelled :=
  { name := "Oona's Blackguard", cost := [generic 1, pip .black], types := [.creature],
    subtypes := [faerie, rogue],
    text :=
      [ flying,
        static (entersWithAdditionalCounters
          (each (and [ isCreature, rogue, youControl,
                        isOther ]))
          1 p1p1Counter),
        whenever
          (does (a (and [isCreature, youControl, hasCounter p1p1Counter]))
             (dealDamage (to := a isPlayer) (kind := some .combat)))
          (instruct (that player) (discard (a (cardIn hand)))) ],
    power := stat 1, toughness := stat 1 }
