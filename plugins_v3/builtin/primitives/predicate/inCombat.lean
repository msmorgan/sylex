primitive inCombat (relation : CombatRelation) (counterpart : Option NounPhrase := none) :
    Predicate := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Predicate.inCombat relation counterpart
}
