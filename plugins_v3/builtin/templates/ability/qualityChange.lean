/-- A quality change: `op` applied to `payload` on `subject`'s characteristics. -/
template qualityChange (subject : NounPhrase) (op : QualityOp) (payload : QualityPayload) :
    StaticSpec := {
  -- TODO(spelling): A quality change: `op` applied to `payload` on `subject`'s characteristics.
  body :=
    characteristicChange subject (Semantics.QualityPayload.edits op payload)
}
