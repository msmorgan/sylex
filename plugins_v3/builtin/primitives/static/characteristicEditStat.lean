primitive characteristicEditStat (op : Semantics.QualityOp) (axis : Semantics.CharacteristicStat) (amount : Semantics.Amount) : Semantics.CharacteristicEdit := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.CharacteristicEdit.stat op axis amount
}
