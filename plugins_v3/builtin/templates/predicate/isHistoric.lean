/-- "historic": an artifact, legendary, or Saga [CR#700.6]. -/
template isHistoric : Predicate := {
  spelling := "historic",
  body :=
    or
      [hasType Semantics.CardType.artifact,
        hasSupertype .legendary,
        hasSubtype saga]
}
