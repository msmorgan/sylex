/-- The end of the specified part of the named turn. -/
primitive endOf (part : TurnPart) (whose : Option NounPhrase) (which : TurnRef) :
    GameEvent := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.GameEvent.endOf part whose which
}
