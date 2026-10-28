/-- The beginning of the specified part of the named turn. -/
primitive beginningOf (part : TurnPart) (whose : Option NounPhrase) (which : TurnRef) :
    GameEvent := {
  body :=
    Semantics.Primitives.GameEvent.beginningOf part whose which
}
