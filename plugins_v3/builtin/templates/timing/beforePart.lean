/-- "before <whose> <part>" -/
template beforePart (part : TurnPart) (whose : Option NounPhrase := none) : Timing := {
  spelling := "before <whose> <part>",
  body :=
    Semantics.Templates.Timing.timingRelativeToEvent Semantics.EventSide.before
      (Semantics.Templates.GameEvent.beginningOf part whose Semantics.Templates.TurnRef.turnRefCurrent)
}
