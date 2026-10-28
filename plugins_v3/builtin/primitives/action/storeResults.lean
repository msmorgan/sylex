/-- Store die results on the subject. -/
primitive storeResults (on : NounPhrase) : Action := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Action.storeResults on
}
