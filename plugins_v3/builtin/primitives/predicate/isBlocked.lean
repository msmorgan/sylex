/-- "blocked": an attacking creature that blockers were declared for, or that an effect made
blocked, until it is removed from combat [CR#509.1h,506.4]. -/
-- [CR#509.1h,506.4]
primitive isBlocked : Predicate := {
  spelling := "blocked",
  body :=
    Semantics.Primitives.Predicate.inCombat Semantics.CombatRelation.blockedBy Option.none
}
