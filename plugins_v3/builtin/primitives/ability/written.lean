/-- "<ability>", lost as written -/
primitive written (ability : Ability) : AbilityLost := {
  spelling := "<ability>",
  body :=
    Semantics.Primitives.AbilityLost.written ability
}
