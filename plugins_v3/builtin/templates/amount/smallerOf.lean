/-- "the smaller of <a> and <b>", "<a> or <b>, whichever is smaller" [CR#704.5q]. -/
-- [CR#704.5q]
template smallerOf (a b : Amount) : Amount := {
  spelling := "the smaller of <a> and <b>",
  body :=
    aggregateOf Semantics.AggregateOp.min [a, b]
}
