primitive relate (subject : NounPhrase) (label : RelationLabel) (counterpart : NounPhrase) :
    Action := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Action.relate subject label counterpart
}
