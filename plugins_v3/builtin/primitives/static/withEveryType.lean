/-- "with every <space> type" -/
primitive withEveryType (space : SubtypeSpace) : WrittenQuality := {
  spelling := "with every <space> type",
  body :=
    Semantics.Primitives.WrittenQuality.everyTypeOf space
}
