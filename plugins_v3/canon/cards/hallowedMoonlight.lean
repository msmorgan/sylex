/-- Hallowed Moonlight -/
card hallowedMoonlight : Spelled :=
  { name := "Hallowed Moonlight", cost := [generic 1, pip .white], types := [.instant],
    text :=
      [ spell (sequentially
          [ replaceEvent (would (a (and [isCreature, not (did any (cast which) thisObject)])) enter)
              (instead (exile it))
              untilEndOfTurn,
            draw 1 ]) ] }
