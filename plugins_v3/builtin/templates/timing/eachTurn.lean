/-- "each turn": a rate adjunct, the deontic counted afresh in every turn. -/
template eachTurn : Timing := {
  spelling := "each turn",
  body :=
    inPart .turn none (eachTime none)
}
