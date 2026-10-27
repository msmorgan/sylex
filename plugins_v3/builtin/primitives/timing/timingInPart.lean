primitive timingInPart (part : Semantics.TurnPart) (whose : Option Semantics.NounPhrase) (which : Semantics.TurnRef) : Semantics.Timing := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.Timing.inPart part whose which
}
