/-- Authoring primitive for `QualityPayload.quality`. -/
primitive quality (quality : WrittenQuality) : QualityPayload := {
  body := Semantics.Primitives.QualityPayload.quality quality
}
