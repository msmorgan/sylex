/-- "spell": an object on the stack that is not an ability [CR#112.1,113.1]. -/
-- [CR#112.1,113.1]
template isSpell : Predicate := {
  spelling := "spell",
  body :=
    Semantics.Templates.Predicate.and
      [Semantics.Templates.Predicate.in_ Semantics.Templates.Location.stack,
        Semantics.Templates.Predicate.not (Semantics.Templates.Predicate.abilityHead Semantics.AbilityClass.anyOnStack)]
}
