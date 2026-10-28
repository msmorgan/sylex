primitive typeLine (op : QualityOp) (changes : TypeLineChanges) : CharacteristicEdit := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.CharacteristicEdit.typeLine op changes
}
