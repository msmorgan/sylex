/-- "all but <amount> of that damage" -/
primitive PreventCut.allBut (amount : Amount) : PreventCut := {
  spelling := "all but <amount> of that damage",
  body :=
    Semantics.Primitives.PreventCut.allBut amount
}
