/-- "each combat": a rate adjunct over every combat phase [CR#506.1]. -/
-- [CR#506.1]
template eachCombat : Timing := {
  spelling := "each combat",
  body :=
    timingInPart .combat none (eachTime none)
}
