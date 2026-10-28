/-- "<spec> during [<whose>] untap step" -/
template duringUntapStep (whose : Option NounPhrase) (spec : StaticSpec) : StaticSpec := {
  -- TODO(spelling): Include or omit the clauses supplied by whose.
  body :=
    partScope untapStep whose spec
}
