/-- "unattach <subject>" -/
template unattach (subject : NounPhrase) : Action := {
  spelling := "unattach <subject>",
  body := attachment .unattached subject none
}
