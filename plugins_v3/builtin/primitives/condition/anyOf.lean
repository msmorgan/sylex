/-- "<trait> or <trait>" -/
primitive anyOf (traits : List DeckTrait) : DeckTrait := {
  -- TODO(spelling): Context-dependent wording: <trait> or <trait>; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Primitives.DeckTrait.anyOf traits
}
