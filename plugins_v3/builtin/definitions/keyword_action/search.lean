/-- "search <scope>", or "search <scope> for <for_>" (`Definition`, [CR#701.23a]).
The `for_` parameter is deliberately unbound: its explicit `inCaller` opts out of generated
identity. Whether the search selection should be bound is an open owner question; this
preserves the existing expansion. -/
action search (scope : Location) (for_ : NounPhrase := any) : Action := {
  -- TODO(spelling): Omit the 'for' phrase when for_ is any; otherwise print the search selection.
  lexical := { verbs := [{ lemma := "search", frames := [(TransitiveWithPP ["for"]), Monotransitive, (Prepositional ["for"])] }] },
  body :=
    actionSearch scope (.inCaller for_)
}
