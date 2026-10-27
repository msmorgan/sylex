/-- "The next time <clause> would happen, <replacement> instead [duration]." -/
template replaceNextEvent (clause : WouldClause) (replacement : Replacement)
    (duration : Option Duration) : Action := {
  -- TODO(spelling): Context-dependent wording: The next time <clause> would happen, <replacement> instead [duration].; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Templates.Action.establish
      (Semantics.Templates.StaticSpec.replace clause replacement Option.none Semantics.ReplUse.nextTimeOnly Option.none) duration
}
