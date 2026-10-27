/-- "choose <subject>" -/
primitive choose (subject : NounPhrase) (disclosure : Disclosure := .openly) : Action := {
  -- TODO(spelling): choose <subject>; wording varies with disclosure, requiring optional clauses, inflection or structure-dependent phrasing.
  body :=
    Semantics.Primitives.Action.choose Option.none subject disclosure Option.none
}
