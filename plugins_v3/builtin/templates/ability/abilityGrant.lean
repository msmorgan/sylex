/-- "<subject> has <ability>" -/
template abilityGrant (subject : NounPhrase) (ability : Ability) : StaticSpec := {
  spelling := "<subject> has <ability>",
  body :=
    Semantics.Templates.StaticSpec.characteristicChange subject [Semantics.Templates.CharacteristicEdit.characteristicEditAddedAbilities [ability]]
}
