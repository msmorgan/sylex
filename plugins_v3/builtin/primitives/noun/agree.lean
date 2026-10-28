/-- An Agreement: constrain the members of a group to share, or not share, a value on the given
axis. -/
primitive agree (axis : QualitySort) (agreement : Agreement) (group : NounPhrase) :
    NounPhrase := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.NounPhrase.agree axis agreement group
}
