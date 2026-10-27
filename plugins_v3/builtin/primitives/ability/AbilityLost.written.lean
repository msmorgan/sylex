/-- "<ability>", lost as written -/
primitive AbilityLost.written (ability : Ability) : AbilityLost := {
  spelling := "<ability>",
  body :=
    Semantics.Primitives.AbilityLost.written ability
}
