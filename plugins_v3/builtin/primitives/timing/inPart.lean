primitive inPart (part : TurnPart) (whose : Option NounPhrase) (which : TurnRef) : Timing := {
  body := Semantics.Primitives.Timing.inPart part whose which
}
