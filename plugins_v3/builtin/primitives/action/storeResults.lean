/-- Store die results on the subject. -/
primitive storeResults (on : NounPhrase) : Action := {
  body :=
    Semantics.Primitives.Action.storeResults on
}
