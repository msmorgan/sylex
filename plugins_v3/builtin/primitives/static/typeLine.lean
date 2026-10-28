primitive typeLine (op : QualityOp) (changes : TypeLineChanges) : CharacteristicEdit := {
  body := Semantics.Primitives.CharacteristicEdit.typeLine op changes
}
