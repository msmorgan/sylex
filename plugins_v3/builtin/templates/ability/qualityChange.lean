/-- A quality change: `op` applied to `payload` on `subject`'s characteristics. -/
template qualityChange (subject : NounPhrase) (op : QualityOp) (payload : QualityPayload) :
    StaticSpec := {
  -- TODO(spelling): The op and payload select replacement, addition, or removal wording and any
  -- retained-type clause.
  body :=
    characteristicChange subject (Semantics.QualityPayload.edits op payload)
}
