primitive ofColor (color : ColorTerm) : PipPaid := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.PipPaid.ofColor color
}
