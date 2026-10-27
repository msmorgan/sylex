primitive isBlocking : Predicate := {
  spelling := "blocking",
  body :=
    Semantics.Primitives.Predicate.inCombat Semantics.CombatRelation.blockerOf Option.none
}
