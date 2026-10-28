/-- "<trait> or <trait>" -/
primitive anyOf (traits : List DeckTrait) : DeckTrait := {
  body :=
    Semantics.Primitives.DeckTrait.anyOf traits
}
