/-- "<subj> <action>s": an action event, in the voice `subj` carries. -/
primitive does (subj : Subject) (action : Action) : GameEvent := {
  -- TODO(spelling): <subj> <action>s; string substitution cannot inflect verbs, nouns, or ordinals.
  body :=
    Semantics.Primitives.GameEvent.does subj action
}
