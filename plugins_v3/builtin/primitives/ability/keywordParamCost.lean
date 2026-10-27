primitive keywordParamCost (cost : Semantics.Cost) : Semantics.KeywordParam := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.KeywordParam.cost cost
}
