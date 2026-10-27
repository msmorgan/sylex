/-- Gather Specimens -/
card gatherSpecimens : Spelled :=
  { name := "Gather Specimens", cost := [generic 3, pip .blue, pip .blue, pip .blue],
    types := [.instant],
    text :=
      [ spell
          (establishThroughout
             (replace (would (a isCreature) (enter (riders := [under anOpponent])))
                (instead (instruct it (enter (riders := [under you])))))
             thisTurn) ] }
