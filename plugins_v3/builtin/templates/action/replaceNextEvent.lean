/-- "The next time <clause> would happen, <replacement> instead [duration]." -/
template replaceNextEvent (clause : WouldClause) (replacement : Replacement)
    (duration : Option Duration) : Action := {
  -- TODO(spelling): Context-dependent wording: The next time <clause> would happen, <replacement> instead [duration].; string substitution cannot express its omitted or inflected parts.
  body :=
    establish
      (replace clause replacement none .nextTimeOnly none) duration
}
