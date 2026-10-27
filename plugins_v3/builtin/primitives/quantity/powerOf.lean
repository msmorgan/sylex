primitive powerOf (subject : NounPhrase) : Amount := {
  spelling := "<subject>'s power",
  body :=
    Semantics.Primitives.Amount.statOf (Semantics.ProjAxis.stat Semantics.Stat.power) subject
}
