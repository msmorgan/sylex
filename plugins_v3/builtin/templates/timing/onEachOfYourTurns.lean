/-- "on each of your turns", "during each of your turns" -/
template onEachOfYourTurns : Timing := {
  spelling := "on each of your turns",
  body :=
    timingInPart Semantics.TurnPart.turn (some you)
      (eachTime none)
}
