primitive inPart (part : TurnPart) (whose : Option NounPhrase) (which : TurnRef) : Timing := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.Timing.inPart part whose which
}
