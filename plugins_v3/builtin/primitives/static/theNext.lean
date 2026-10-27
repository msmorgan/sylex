/-- "the next <amount> damage" [CR#615.7] -/
-- [CR#615.7]
primitive theNext (amount : Amount) : PreventCut := {
  spelling := "the next <amount> damage",
  body :=
    Semantics.Primitives.PreventCut.shield amount
}
