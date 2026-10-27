/-- Thranduil's Company -/
card thranduilsCompany : Spelled :=
  { name := "Thranduil's Company", cost := [generic 2, pip .green, pip .blue],
    types := [.creature], subtypes := [elf, soldier],
    text :=
      [ static (onlyWhile (can you (play (counted (additional (exactly 1)) isLand)))
          (exists_ (and [ and [isCreature, isOther], elf,
                           youControl ]))),
        landfall
          (whenever (does (a (and [isLand, youControl])) enter)
            (sequentially
              [ putCounters 2 p1p1Counter
                  (target (and [isCreature, youControl])),
                gain it vigilance untilEndOfTurn ])) ],
    power := stat 3, toughness := stat 4 }
