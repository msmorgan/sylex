/-- An ability that states a Static Spec. -/
primitive static (spec : StaticSpec) : Ability := {
  body :=
    Semantics.Primitives.Ability.static spec
}
