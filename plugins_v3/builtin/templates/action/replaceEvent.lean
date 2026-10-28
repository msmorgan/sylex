/-- "If <clause> would happen, <replacement> instead [duration]." -/
template replaceEvent (clause : WouldClause) (replacement : Replacement)
    (duration : Option Duration) : Action := {
  -- TODO(spelling): Context-dependent wording: If <clause> would happen, <replacement> instead [duration].; string substitution cannot express its omitted or inflected parts.
  body :=
    establish
      (replace clause replacement none .repeatedly none) duration
}
