/-- "<subject> loses all abilities [except <except>]" -/
template allAbilityLoss (subject : NounPhrase) (except : Option Predicate := none) :
    StaticSpec := {
  -- TODO(spelling): Context-dependent wording: <subject> loses all abilities [except <except>]; string substitution cannot express its omitted or inflected parts.
  body :=
    characteristicChange subject
      [removedAbilities (allExcept except)]
}
