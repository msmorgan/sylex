/-- `op` written once over established quality wording: the edits the payload writes. -/
primitive payload (op : QualityOp) (written : QualityPayload) : CharacteristicEdit := {
  body := Semantics.Primitives.CharacteristicEdit.payload op written
}
