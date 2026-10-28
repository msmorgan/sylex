/-- "<group> with different names" -/
template withDifferentNames (group : NounPhrase) : NounPhrase := {
  spelling := "<group> with different names",
  body := agree .cardName .differ group
}
