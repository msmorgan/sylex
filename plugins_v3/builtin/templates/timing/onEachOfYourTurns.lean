/-- "on each of your turns", "during each of your turns" -/
template onEachOfYourTurns : Timing := {
  spelling := "on each of your turns",
  body :=
    inPart .turn (some you)
      (eachTime none)
}
