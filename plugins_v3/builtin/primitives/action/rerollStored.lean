/-- The actor rerolls the specified quantity of stored die results. -/
primitive rerollStored (quantity : Quantity) (whose : NounPhrase) : Action := {
  body :=
    Semantics.Primitives.Action.rerollStored quantity whose
}
