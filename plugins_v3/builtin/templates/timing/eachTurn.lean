/-- "each turn": a rate adjunct, the deontic counted afresh in every turn. -/
template eachTurn : Timing := {
  spelling := "each turn",
  body :=
    Semantics.Templates.Timing.timingInPart Semantics.TurnPart.turn Option.none (Semantics.Templates.TurnRef.eachTime Option.none)
}
