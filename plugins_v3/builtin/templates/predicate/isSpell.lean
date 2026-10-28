/-- "spell": an object on the stack that is not an ability [CR#112.1,113.1]. -/
-- [CR#112.1,113.1]
template isSpell : Predicate := {
  spelling := "spell",
  body :=
    and
      [in_ stack,
        not (abilityHead Semantics.AbilityClass.anyOnStack)]
}
