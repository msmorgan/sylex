/-- "the chosen <quality>" -/
template chosenQuality (quality : Predicate) : QualityPayload := {
  spelling := "the chosen <quality>",
  body :=
    Semantics.Templates.QualityPayload.qualityPayloadQuality (Semantics.Templates.WrittenQuality.withQuality quality)
}
