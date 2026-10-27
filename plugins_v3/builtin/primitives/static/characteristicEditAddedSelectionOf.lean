primitive characteristicEditAddedSelectionOf (source : Semantics.NounPhrase) (selection : Semantics.AbilitySelection) : Semantics.CharacteristicEdit := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.CharacteristicEdit.addedSelectionOf source selection
}
