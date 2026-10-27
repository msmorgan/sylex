primitive defendingPlayer : NounPhrase := {
  spelling := "the defending player",
  body :=
    Semantics.Primitives.NounPhrase.combatPlayer Semantics.CombatRole.defending
}
