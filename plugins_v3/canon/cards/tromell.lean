/-- Tromell, Seymour's Butler -/
card tromell : Spelled :=
  { name := "Tromell, Seymour's Butler", cost := [generic 2, pip .green],
    supertypes := [.legendary], types := [.creature],
    subtypes := [elf, advisor],
    text :=
      [ static (entersWithAdditionalCounters
          (each (and [isCreature, isNontoken, youControl, isOther]))
          1 p1p1Counter),
        activated (compound [[generic 1], tapSymbol])
          (sequentially
            [ repeatTimes (letter .x) proliferate,
              define .x
                (countOf (and [ isNontoken, isCreature, youControl,
                                 did which enter (within soFarThisTurn) ])) ]) ],
    power := stat 2, toughness := stat 3 }
