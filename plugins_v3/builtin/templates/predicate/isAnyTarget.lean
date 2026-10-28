/-- "any target" [CR#115.4]. -/
template isAnyTarget : Predicate := {

  -- TODO(spelling): any target; the predicate supplies target eligibility, while the determiner
  -- supplies any/target, so printing it here would duplicate the enclosing noun phrase.
  body :=
    or
      [hasType Semantics.CardType.creature, hasType Semantics.CardType.planeswalker,
        hasType Semantics.CardType.battle, isPlayer]
}
