/-- "choose <chosen> starting with <first>" -/
template chooseStartingWith (first chosen : NounPhrase) (disclosure : Disclosure := .openly)
    (when : Option Timing := none) : Action := {
  -- TODO(spelling): choose <chosen> starting with <first>; wording varies with disclosure, when, requiring optional clauses, inflection or structure-dependent phrasing.
  body := choose chosen disclosure (some first) when
}
