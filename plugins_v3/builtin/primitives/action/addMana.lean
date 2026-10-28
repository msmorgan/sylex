/-- Add mana to the actor's mana pool. -/
primitive addMana (amount : Amount) (produced : ProducedMana)
    (riders : List ManaRider := []) : Action := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Action.addMana amount produced riders
}
