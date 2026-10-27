/-- "every <space> type" -/
template everyTypeOf (space : SubtypeSpace) : QualityPayload := {
  spelling := "every <space> type",
  body :=
    Semantics.Templates.QualityPayload.qualityPayloadQuality (Semantics.Templates.WrittenQuality.withEveryType space)
}
