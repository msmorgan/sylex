/-- "during <whose> <part>" -/
template duringPart (part : TurnPart) (whose : Option NounPhrase := none) : Timing := {
  spelling := "during <whose> <part>",
  body :=
    Semantics.Templates.Timing.timingInPart part whose Semantics.Templates.TurnRef.turnRefCurrent
}
