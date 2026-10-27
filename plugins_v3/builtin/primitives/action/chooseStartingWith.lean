/-- "choose <chosen> starting with <first>" -/
primitive chooseStartingWith (first chosen : NounPhrase) (disclosure : Disclosure := .openly)
    (when : Option Timing := none) : Action := {
  -- TODO(spelling): choose <chosen> starting with <first>; wording varies with disclosure, when, requiring optional clauses, inflection or structure-dependent phrasing.
  body :=
    Semantics.Primitives.Action.choose (Option.some first) chosen disclosure when
}
