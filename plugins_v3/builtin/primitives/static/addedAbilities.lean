/-- Authoring primitive for `CharacteristicEdit.addedAbilities`. -/
primitive addedAbilities (abilities : List Ability) : CharacteristicEdit := {
  body := Semantics.Primitives.CharacteristicEdit.addedAbilities abilities
}
