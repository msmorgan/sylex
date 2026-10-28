/-- "attached to <host>" -/
template attachedTo (host : NounPhrase) : Predicate := {
  spelling := "attached to <host>",
  body := attachment .attachment (some host) none
}
