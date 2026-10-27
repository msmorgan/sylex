/-- "historic": an artifact, legendary, or Saga [CR#700.6]. -/
template isHistoric : Predicate := {
  spelling := "historic",
  body :=
    Semantics.Templates.Predicate.or
      [Semantics.Templates.Predicate.hasType Semantics.CardType.artifact,
        Semantics.Templates.Predicate.hasSupertype Semantics.Supertype.legendary,
        Semantics.Templates.Predicate.hasSubtype Semantics.Definitions.saga]
}
