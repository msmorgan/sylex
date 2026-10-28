/-- Repeat an Action for each distinct Kind of a Sort in a Domain. -/
primitive doForEachKind (axis : KindAxis) (domain : Option NounPhrase := none)
    (sort : QualitySort) (body : Action) : Action := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Action.doForEachKind axis domain sort body
}
