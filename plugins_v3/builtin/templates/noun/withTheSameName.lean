/-- "<group> with the same name" -/
template withTheSameName (group : NounPhrase) : NounPhrase := {
  spelling := "<group> with the same name",
  body := agree cardName share group
}
