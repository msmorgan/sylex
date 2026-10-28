/-- Add mana to the actor's mana pool. -/
primitive addMana (amount : Amount) (produced : ProducedMana)
    (riders : List ManaRider := []) : Action := {
  body :=
    Semantics.Primitives.Action.addMana amount produced riders
}
