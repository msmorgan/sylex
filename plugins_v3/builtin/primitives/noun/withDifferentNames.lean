/-- "<group> with different names" -/
primitive withDifferentNames (group : NounPhrase) : NounPhrase := {
  spelling := "<group> with different names",
  body :=
    Semantics.Primitives.NounPhrase.agree Semantics.QualitySort.cardName Semantics.Agreement.differ group
}
