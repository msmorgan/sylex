primitive doForEachKind (axis : KindAxis) (sort : QualitySort) (body : Action)
    (domain : Option NounPhrase := none) : Action := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Action.doForEachKind axis domain sort body
}
