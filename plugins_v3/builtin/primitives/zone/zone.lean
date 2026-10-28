/-- A zone location with an optional owner or controller scope. -/
primitive zone (zone : Zone) (scope : Option NounPhrase) : Location := {
  body :=
    Semantics.Primitives.Location.zone zone scope
}
