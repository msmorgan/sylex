primitive blocking (move : AttachMove) (attacker : NounPhrase) : CombatUpdate := {
  body := Semantics.Primitives.CombatUpdate.blocking move attacker
}
