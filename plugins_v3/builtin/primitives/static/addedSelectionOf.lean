primitive addedSelectionOf (source : NounPhrase)
    (selection : AbilitySelection) : CharacteristicEdit := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.CharacteristicEdit.addedSelectionOf source selection
}
