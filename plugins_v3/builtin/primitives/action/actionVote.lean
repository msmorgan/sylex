primitive actionVote (first : Option NounPhrase) (disclosure : Disclosure) (ballot : Ballot) (times : Option Quantity) : Action := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.Action.vote first disclosure ballot times
}
