/-- Distribute an amount of damage or counters among the selected objects or players. -/
primitive distribute (verb : DividedVerb) (amount : Amount) (among : NounPhrase) : Action := {
  body :=
    Semantics.Primitives.Action.distribute verb amount among
}
