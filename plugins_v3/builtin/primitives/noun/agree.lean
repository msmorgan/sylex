primitive agree (axis : QualitySort) (agreement : Agreement) (group : NounPhrase) :
    NounPhrase := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.NounPhrase.agree axis agreement group
}
