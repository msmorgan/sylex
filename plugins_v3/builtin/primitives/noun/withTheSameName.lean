/-- "<group> with the same name" -/
primitive withTheSameName (group : NounPhrase) : NounPhrase := {
  spelling := "<group> with the same name",
  body :=
    Semantics.Primitives.NounPhrase.agree Semantics.QualitySort.cardName Semantics.Agreement.share group
}
