/-- The beginning of the specified part of the named turn. -/
primitive beginningOf (part : TurnPart) (whose : Option NounPhrase) (which : TurnRef) :
    GameEvent := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.GameEvent.beginningOf part whose which
}
