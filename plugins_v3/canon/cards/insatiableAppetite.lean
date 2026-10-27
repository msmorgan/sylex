/-- Insatiable Appetite: "You may sacrifice a Food. If you do, target creature gets +5/+5 until end
of turn. Otherwise, that creature gets +3/+3 until end of turn." Both arms' targets are chosen as
the spell is cast, whichever arm later runs [CR#601.2c,115.1], so "Otherwise, that creature" reads
the target the "if you do" arm announces. -/
card insatiableAppetite : Spelled :=
  { name := "Insatiable Appetite", cost := [generic 1, pip .green], types := [.instant],
    text :=
      [ spell (may (sacrifice (a food))
          (get (target isCreature) (up 5) (up 5) untilEndOfTurn)
          (get (that creature) (up 3) (up 3) untilEndOfTurn)) ] }
