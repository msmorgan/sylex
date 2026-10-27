/-- "if <player> could begin to cast <subject> by putting it onto the stack from <from_>"
[CR#702.62a,116.2f] -/
-- [CR#702.62a,116.2f]
primitive couldBeginToCast (player subject : NounPhrase) (from_ : Location) : Condition := {
  spelling := "if <player> could begin to cast <subject> by putting it onto the stack from <from_>",
  body :=
    Semantics.Primitives.Condition.couldBeginToCast player subject from_
}
