/-- Animal Attendant -/
card animalAttendant : Spelled :=
  { name := "Animal Attendant", cost := [generic 1, pip .green], types := [.creature],
    subtypes := [human, citizen],
    text :=
      [ activated tapSymbol
          (addMana 1 (ofAnyColor .sameColor)
            [ whenSpentOn .affectsIt false
                (a (and [non human, isCreature, isSpell]))
                (establish
                  (entersWithAdditionalCounters (becomes (that .spell)) 1
                     p1p1Counter)) ]) ],
    power := stat 2, toughness := stat 2 }
