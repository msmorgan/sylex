/-- "choose <chosen> starting with <first>" -/
template chooseStartingWith (first chosen : NounPhrase) (disclosure : Disclosure := .openly)
    (when : Option Timing := none) : Action := {
  -- TODO(spelling): Include or omit the clauses supplied by when.
  body := choose chosen disclosure (some first) when
}
