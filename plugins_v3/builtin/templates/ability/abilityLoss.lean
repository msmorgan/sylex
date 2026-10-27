/-- "<subject> loses <abilities>" -/
template abilityLoss (subject : NounPhrase) (abilities : List AbilityLost) : StaticSpec := {
  spelling := "<subject> loses <abilities>",
  body :=
    Semantics.Templates.StaticSpec.characteristicChange subject
      [Semantics.Templates.CharacteristicEdit.characteristicEditRemovedAbilities (Semantics.Templates.AbilitySelection.abilitySelectionSpecified abilities)]
}
