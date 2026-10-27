/-- Tawnos, the Toymaker -/
card tawnosTheToymaker : Spelled :=
  { name := "Tawnos, the Toymaker", cost := [generic 3, pip .green, pip .blue],
    supertypes := [.legendary], types := [.creature],
    subtypes := [human, artificer],
    text :=
      [ whenever
          (does you (cast (a (and [ or [
              beast,
                             bird ],
                       isCreature, isSpell ]))))
          (may (copyOnStack it (exceptions := [isAlso [.artifact] []]))) ],
    power := stat 3, toughness := stat 5 }
