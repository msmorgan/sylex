primitive characteristicEditColors (op : Semantics.QualityOp) (colors : Semantics.ColorSpec) : Semantics.CharacteristicEdit := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.CharacteristicEdit.colors op colors
}
