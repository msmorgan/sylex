primitive toughnessOf (subject : NounPhrase) : Amount := {
  spelling := "<subject>'s toughness",
  body :=
    Semantics.Primitives.Amount.statOf (Semantics.ProjAxis.stat Semantics.Stat.toughness) subject
}
