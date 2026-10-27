/-- "the <ordinal>", of a counter put: the put that crosses the ordinal's threshold [CR#122.7]. -/
-- [CR#122.7]
primitive theNth (ordinal : Ordinal) : Amount := {
  spelling := "the <ordinal>",
  body :=
    Semantics.Primitives.Amount.theNth ordinal
}
