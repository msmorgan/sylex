primitive shares (axis : QualitySort) (agreement : Agreement) (with_ : NounPhrase) :
    Predicate := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Predicate.shares axis agreement with_
}
