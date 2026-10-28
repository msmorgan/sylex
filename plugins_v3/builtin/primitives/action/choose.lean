/-- "choose <subject>" -/
primitive choose (subject : NounPhrase) (disclosure : Disclosure := openly)
    (first : Option NounPhrase := none) (when : Option Timing := none) : Action := {
  -- TODO(spelling): Include or omit the clauses supplied by first, when.
  body := Semantics.Primitives.Action.choose first subject disclosure when
}
