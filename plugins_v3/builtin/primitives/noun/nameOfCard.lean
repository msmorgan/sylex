/-- "a card name [of <predicate>]" -/
primitive nameOfCard (predicate : Predicate) : ChoiceDomain := {
  -- TODO(spelling): Context-dependent wording: a card name [of <predicate>]; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Primitives.ChoiceDomain.nameOfCard predicate
}
