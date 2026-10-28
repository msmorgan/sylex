primitive stat (op : QualityOp) (axis : CharacteristicStat)
    (amount : Amount) : CharacteristicEdit := {
  body := Semantics.Primitives.CharacteristicEdit.stat op axis amount
}
