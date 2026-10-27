/-- "<subject>'s base power" [CR#208.4b] -/
-- [CR#208.4b]
primitive basePowerOf (subject : NounPhrase) : Amount := {
  spelling := "<subject>'s base power",
  body :=
    Semantics.Primitives.Amount.statOf (Semantics.ProjAxis.stat Semantics.Stat.basePower) subject
}
