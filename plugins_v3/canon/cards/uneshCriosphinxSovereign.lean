/-- Unesh, Criosphinx Sovereign -/
card uneshCriosphinxSovereign : Spelled :=
  { name := "Unesh, Criosphinx Sovereign", cost := [generic 4, pip .blue, pip .blue],
    supertypes := [.legendary], types := [.creature], subtypes := [sphinx],
    text :=
      [ flying,
        static (spellsCost (and [sphinx, isSpell, did you (cast which) thisObject]) (less 2)),
        whenever
          (does (eitherOf thisCreature
            (a (and [sphinx, youControl, isOther]))) enter)
          (sequentially
            [ reveal (topSlice 4),
              instruct anOpponent (separateIntoPiles them 2),
              putInto onePile hand,
              putInto theOther graveyard ]) ],
    power := stat 4, toughness := stat 4 }
