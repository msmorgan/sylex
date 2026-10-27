/-- Pippin's Bravery: "You may sacrifice a Food. If you do, target creature gets +4/+4 until end of
turn. Otherwise, that creature gets +2/+2 until end of turn." The same shape as Insatiable
Appetite. -/
card pippinsBravery : Spelled :=
  { name := "Pippin's Bravery", cost := [pip .green], types := [.instant],
    text :=
      [ spell (may (sacrifice (a food))
          (get (target isCreature) (up 4) (up 4) untilEndOfTurn)
          (get (that creature) (up 2) (up 2) untilEndOfTurn)) ] }
