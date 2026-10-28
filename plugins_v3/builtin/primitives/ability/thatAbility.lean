/-- The ability selected by the given choice reference. -/
primitive thatAbility (ref : ChoiceRef) : Ability := {
  body :=
    Semantics.Primitives.Ability.thatAbility ref
}
