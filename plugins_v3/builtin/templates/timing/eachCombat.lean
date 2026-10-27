/-- "each combat": a rate adjunct over every combat phase [CR#506.1]. -/
-- [CR#506.1]
template eachCombat : Timing := {
  spelling := "each combat",
  body :=
    Semantics.Templates.Timing.timingInPart Semantics.TurnPart.combat Option.none (Semantics.Templates.TurnRef.eachTime Option.none)
}
