template isAttacking : Predicate := {
  spelling := "attacking",
  body := inCombat .attackerOf none
}
