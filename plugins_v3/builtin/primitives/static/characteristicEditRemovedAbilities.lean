primitive characteristicEditRemovedAbilities (selection : Semantics.AbilitySelection) : Semantics.CharacteristicEdit := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.CharacteristicEdit.removedAbilities selection
}
