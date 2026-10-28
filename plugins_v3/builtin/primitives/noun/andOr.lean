/-- "<np> and/or <np>", as "a card named Magnifying Glass and/or a card named Thinking Cap". -/
primitive andOr (phrases : List NounPhrase := []) : NounPhrase := {
  -- TODO(spelling): Context-dependent wording: <np> and/or <np>; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Primitives.NounPhrase.coord Semantics.Coordinator.andOr phrases
}
