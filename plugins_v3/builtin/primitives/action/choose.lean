/-- "choose <subject>" -/
primitive choose (subject : NounPhrase) (disclosure : Disclosure := .openly)
    (first : Option NounPhrase := none) (when : Option Timing := none) : Action := {
  -- TODO(spelling): choose <subject>; wording varies with disclosure, requiring optional clauses, inflection or structure-dependent phrasing.
  body := Semantics.Primitives.Action.choose first subject disclosure when
}
