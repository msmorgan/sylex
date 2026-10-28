/-- "any target" [CR#115.4]. -/
template isAnyTarget : Predicate := {

  body :=
    or
      [hasType Semantics.CardType.creature, hasType Semantics.CardType.planeswalker,
        hasType Semantics.CardType.battle, isPlayer]
}
