/-- "mana ability": a Predicate selecting Mana Abilities. -/
template isManaAbility : Predicate := {
  spelling := "mana ability",
  body := abilityHead mana
}
