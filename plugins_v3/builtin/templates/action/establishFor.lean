/-- Several static clauses sharing one subject, as one action. -/
template establishFor (subject : NounPhrase) (parts : List StaticSpec)
    (duration : Option Duration := none) : Action := {
  -- TODO(spelling): Several static clauses sharing one subject, as one action.
  body :=
    establish (staticSpecConjunction (some subject) parts) duration
}
