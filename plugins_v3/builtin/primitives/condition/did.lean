/-- "if <subj> <action>ed <within>": a condition always reads a written span. -/
primitive did (subj : Subject) (action : Action) (within : LookbackSpan) :
    Condition := {
  -- TODO(spelling): if <subj> <action>ed <within>; string substitution cannot inflect verbs, nouns, or ordinals.
  body :=
    Semantics.Primitives.Condition.did { subj := subj, action := action, within := some within }
}
