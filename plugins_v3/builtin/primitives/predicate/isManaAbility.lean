primitive isManaAbility : Predicate := {
  spelling := "mana ability",
  body :=
    Semantics.Primitives.Predicate.abilityHead Semantics.AbilityClass.mana
}
