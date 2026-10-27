primitive createObject (count : Amount) (spec : CreationSpec)
    (under : Option NounPhrase := none) : Action := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Action.createObject count spec under
}
