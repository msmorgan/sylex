/-- Frostwielder -/
card frostwielder : Spelled :=
  { name := "Frostwielder", cost := [generic 2, pip .red, pip .red], types := [.creature],
    subtypes := [human, shaman],
    text :=
      [ static
          (replace
            (would
              (a (and [ isCreature,
                        did thisCreature (dealDamage anyAmount which) (within soFarThisTurn) ]))
              die)
            (instead (exile it))),
        activated tapSymbol
          (instruct thisCreature (dealDamage 1 (target isAnyTarget))) ],
    power := stat 1, toughness := stat 2 }
