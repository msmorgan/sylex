/-- "<subject>'s base toughness" [CR#208.4b] -/
-- [CR#208.4b]
primitive baseToughnessOf (subject : NounPhrase) : Amount := {
  spelling := "<subject>'s base toughness",
  body :=
    Semantics.Primitives.Amount.statOf (Semantics.ProjAxis.stat Semantics.Stat.baseToughness) subject
}
