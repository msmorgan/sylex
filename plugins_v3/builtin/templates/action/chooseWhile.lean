/-- "choose <subject> as you <event>" -/
template chooseWhile (subject : NounPhrase) (while_ : Timing) : Action := {
  -- TODO(spelling): Context-dependent wording: choose <subject> as you <event>; string substitution cannot express its omitted or inflected parts.
  body := choose subject .openly none (some while_)
}
