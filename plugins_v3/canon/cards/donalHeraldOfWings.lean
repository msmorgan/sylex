/-- Donal, Herald of Wings -/
card donalHeraldOfWings : Spelled :=
  { name := "Donal, Herald of Wings", cost := [generic 2, pip .blue, pip .blue],
    supertypes := [.legendary], types := [.creature],
    subtypes := [human, wizard],
    text :=
      [ whenever
          (does you (cast (a (and [ isCreature, isSpell, not
              (hasSupertype .legendary),
                       hasKeyword flying ]))))
          (doOnlyOnceEachTurn (may
            (copyOnStack it
              (exceptions := (copyCharacteristics {
                         subtypes := [spirit], power := stat 1,
                         toughness := stat 1 } true))))) ],
    power := stat 3, toughness := stat 3 }
