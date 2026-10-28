/-- Tests sharing or differing values along the selected quality axis. For names, sharing means
at least one name in common; a different name requires at least one name and no names in
common. [CR#201.2a,201.2c] -/
primitive shares (axis : QualitySort) (agreement : Agreement) (with_ : NounPhrase) :
    Predicate := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Predicate.shares axis agreement with_
}
