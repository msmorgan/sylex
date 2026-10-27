primitive conjunction (parts : List StaticSpec) (subject : Option NounPhrase := none) :
    StaticSpec := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.StaticSpec.conjunction subject parts
}
