primitive characteristicEditTypeLine (op : Semantics.QualityOp) (changes : Semantics.TypeLineChanges) : Semantics.CharacteristicEdit := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.CharacteristicEdit.typeLine op changes
}
