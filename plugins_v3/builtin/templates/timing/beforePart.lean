/-- "before <whose> <part>" -/
template beforePart (part : TurnPart) (whose : Option NounPhrase := none) : Timing := {
  spelling := "before <whose> <part>",
  body :=
    timingRelativeToEvent .before
      (beginningOf part whose turnRefCurrent)
}
