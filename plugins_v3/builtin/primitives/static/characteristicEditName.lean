primitive characteristicEditName (op : Semantics.QualityOp) (name : String) : Semantics.CharacteristicEdit := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.CharacteristicEdit.name op name
}
