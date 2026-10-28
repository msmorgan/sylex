/-- An ability that states a Static Spec. -/
primitive static (spec : StaticSpec) : Ability := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Ability.static spec
}
