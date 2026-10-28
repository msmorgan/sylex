/-- A zone location with an optional owner or controller scope. -/
primitive zone (zone : Zone) (scope : Option NounPhrase) : Location := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Location.zone zone scope
}
