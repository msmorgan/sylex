primitive characteristicEditAddedAbilities (abilities : List Semantics.Ability) : Semantics.CharacteristicEdit := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.CharacteristicEdit.addedAbilities abilities
}
