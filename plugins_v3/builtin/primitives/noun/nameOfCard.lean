/-- "a card name [of <predicate>]" -/
primitive nameOfCard (predicate : Predicate) : ChoiceDomain := {
  body :=
    Semantics.Primitives.ChoiceDomain.nameOfCard predicate
}
