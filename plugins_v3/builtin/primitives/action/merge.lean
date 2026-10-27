primitive merge (subject onto : NounPhrase) (position : MergePosition) : Action := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Action.merge subject onto position
}
