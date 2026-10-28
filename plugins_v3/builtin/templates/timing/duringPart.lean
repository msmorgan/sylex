/-- "during <whose> <part>" -/
template duringPart (part : TurnPart) (whose : Option NounPhrase := none) : Timing := {
  spelling := "during <whose> <part>",
  body :=
    timingInPart part whose turnRefCurrent
}
