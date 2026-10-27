/-- Several static clauses sharing one subject, as one action. -/
template establishFor (subject : NounPhrase) (parts : List StaticSpec)
    (duration : Option Duration := none) : Action := {
  -- TODO(spelling): Several static clauses sharing one subject, as one action.
  body :=
    Semantics.Templates.Action.establish (Semantics.Templates.StaticSpec.staticSpecConjunction (Option.some subject) parts) duration
}
