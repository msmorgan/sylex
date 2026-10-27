/-- Star Pupil -/
card starPupil : Spelled :=
  { name := "Star Pupil", cost := [pip .white], types := [.creature],
    subtypes := [human, wizard],
    text :=
      [ static (entersWithCounters thisCreature 1 p1p1Counter),
        when (does thisCreature die)
            (doForEachKind .counterKind .counterKind
              (putCounters (statOf .boundCounter it) thatKind
                (target (and [isCreature, youControl])))
              (domain := it)) ],
    power := stat 0, toughness := stat 0 }
