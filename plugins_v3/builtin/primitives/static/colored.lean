/-- "<colors>", as "becomes blue" -/
primitive colored (colors : ColorSpec) : QualityPayload := {
  spelling := "<colors>",
  body :=
    Semantics.Primitives.QualityPayload.colored colors
}
