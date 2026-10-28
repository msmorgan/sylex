/-- The end of the specified part of the named turn. -/
primitive endOf (part : TurnPart) (whose : Option NounPhrase) (which : TurnRef) :
    GameEvent := {
  body :=
    Semantics.Primitives.GameEvent.endOf part whose which
}
