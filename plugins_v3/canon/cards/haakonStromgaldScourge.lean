/-- Haakon, Stromgald Scourge -/
card haakonStromgaldScourge : Spelled :=
  { name := "Haakon, Stromgald Scourge", cost := [generic 1, pip .black, pip .black],
    supertypes := [.legendary], types := [.creature],
    subtypes := [zombie, knight],
    text :=
      [ static (can you (cast this_ (from_ := yourGraveyard))),
        static (cant you (cast this_ (from_ := some (not yourGraveyard)))),
        static (onlyWhile
                  (can you
                     (cast (bare (and [isSpell, knight]))
                        (from_ := yourGraveyard))) (is this_ (in_ battlefield))),
        when (does thisCreature die) (loseLife 2) ],
    power := stat 3, toughness := stat 3 }
