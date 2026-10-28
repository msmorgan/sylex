/-- The actor rerolls the specified quantity of stored die results. -/
primitive rerollStored (quantity : Quantity) (whose : NounPhrase) : Action := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Action.rerollStored quantity whose
}
