/-- "When that mana is spent to cast <spell>, <says>"; "that spell <says>" (`mode`). -/
primitive whenSpentOn (mode : SpentMode) (only : Bool) (spell : NounPhrase) (says : Action) :
    ManaRider := {
  -- TODO(spelling): The mode and only flag choose cast, activation, or resolution wording and its
  -- restriction.
  body :=
    Semantics.Primitives.ManaRider.onSpent mode only spell says
}
