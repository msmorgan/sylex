primitive actionDoForEachKind (axis : Semantics.KindAxis) (domain : Option Semantics.NounPhrase) (sort : Semantics.QualitySort) (body : Semantics.Action) : Semantics.Action := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.Action.doForEachKind axis domain sort body
}
