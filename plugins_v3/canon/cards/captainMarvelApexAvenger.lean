/-- Captain Marvel, Apex Avenger -/
card captainMarvelApexAvenger : Spelled :=
  { name := "Captain Marvel, Apex Avenger", cost := [generic 5, pip .red, pip .white],
    supertypes := [.legendary], types := [.creature],
    subtypes := [human, kree, hero],
    text :=
      [ flying, doubleStrike, indestructible,
        whenever
          (does (by_ you) (putCounters anyAmount anyKind (a (and [isCreature, isOther]))))
          (if_ := is it (not kree))
          (may (putCounters thatMuch thoseCounters this_)) ],
    power := stat 4, toughness := stat 4 }
