primitive isAttacking : Predicate := {
  spelling := "attacking",
  body :=
    Semantics.Primitives.Predicate.inCombat Semantics.CombatRelation.attackerOf Option.none
}
