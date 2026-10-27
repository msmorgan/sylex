primitive qualityPayloadQuality (quality : Semantics.WrittenQuality) : Semantics.QualityPayload := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.QualityPayload.quality quality
}
