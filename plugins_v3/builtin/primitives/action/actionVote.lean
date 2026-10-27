primitive actionVote (first : Option Semantics.NounPhrase) (disclosure : Semantics.Disclosure) (ballot : Semantics.Ballot) (times : Option Semantics.Quantity) : Semantics.Action := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.Action.vote first disclosure ballot times
}
