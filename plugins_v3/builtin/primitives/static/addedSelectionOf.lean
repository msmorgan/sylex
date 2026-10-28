/-- Adds the abilities selected from `source`, as backup grants the non-backup abilities printed
below it. [CR#702.165a] -/
primitive addedSelectionOf (source : NounPhrase)
    (selection : AbilitySelection) : CharacteristicEdit := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.CharacteristicEdit.addedSelectionOf source selection
}
