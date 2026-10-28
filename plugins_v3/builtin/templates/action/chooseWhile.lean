/-- "choose <subject> as you <event>" -/
template chooseWhile (subject : NounPhrase) (while_ : Timing) : Action := {
  spelling := "choose <subject> <while_>",
  body := choose subject .openly none (some while_)
}
