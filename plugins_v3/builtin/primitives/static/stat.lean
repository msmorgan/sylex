primitive stat (op : QualityOp) (axis : CharacteristicStat)
    (amount : Amount) : CharacteristicEdit := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.CharacteristicEdit.stat op axis amount
}
