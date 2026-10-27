primitive related (label : RelationLabel) (counterpart : Option NounPhrase := none) :
    Predicate := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Predicate.related label counterpart
}
