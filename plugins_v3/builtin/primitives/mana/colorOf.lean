primitive colorOf (object : NounPhrase) : ColorTerm := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.ColorTerm.colorOf object
}
