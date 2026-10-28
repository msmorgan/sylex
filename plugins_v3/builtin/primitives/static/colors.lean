primitive colors (op : QualityOp) (colors : ColorSpec) : CharacteristicEdit := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.CharacteristicEdit.colors op colors
}
