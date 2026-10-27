/-- "any target" [CR#115.4]. -/
template isAnyTarget : Predicate := {

  -- TODO(spelling): any target; the predicate supplies target eligibility, while the determiner
  -- supplies any/target, so printing it here would duplicate the enclosing noun phrase.
  body :=
    Semantics.Templates.Predicate.or
      [Semantics.Templates.Predicate.hasType Semantics.CardType.creature, Semantics.Templates.Predicate.hasType Semantics.CardType.planeswalker,
        Semantics.Templates.Predicate.hasType Semantics.CardType.battle, Semantics.Templates.Predicate.isPlayer]
}
