/-- Authoring primitive for `Predicate.inCombat`. -/
primitive inCombat (relation : CombatRelation) (counterpart : Option NounPhrase := none) :
    Predicate := {
  body :=
    Semantics.Primitives.Predicate.inCombat relation counterpart
}
