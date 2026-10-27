/-- Unwinding Clock -/
card unwindingClock : Spelled :=
  { name := "Unwinding Clock", cost := [generic 4], types := [.artifact],
    text :=
      [ static (untapDuring (allOf (and [isArtifact, youControl]))
          (each isOtherPlayer)) ] }
