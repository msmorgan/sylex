/-- "search <scope>", or "search <scope> for <for_>" (`Definition`, [CR#701.23a]).
The `for_` parameter is deliberately unbound: its explicit `inCaller` opts out of generated
identity. Whether the search selection should be bound is an open owner question; this
preserves the existing expansion. -/
action search (scope : Location) (for_ : NounPhrase := any) : Action := {
  -- TODO(spelling): search <scope> [for <for_>]; string substitution cannot branch on optional
  -- fields or supply contextual agreement and morphology.
  lexical := { verbs := [{ lemma := "search", frames := [(TransitiveWithPP ["for"]), Monotransitive, (Prepositional ["for"])] }] },
  body :=
    actionSearch scope (Semantics.NounPhrase.inCaller for_)
}
