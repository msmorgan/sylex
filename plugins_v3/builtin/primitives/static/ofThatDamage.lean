/-- "<amount> of that damage" -/
primitive ofThatDamage (amount : Amount) : PreventCut := {
  spelling := "<amount> of that damage",
  body :=
    Semantics.Primitives.PreventCut.some amount
}
