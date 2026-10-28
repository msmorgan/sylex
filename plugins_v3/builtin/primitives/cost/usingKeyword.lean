/-- "using its <keyword> ability": the alternative cost that keyword ability offers
[CR#601.2b,601.2f]. -/
-- [CR#601.2b,601.2f]
primitive usingKeyword (label : KeywordLabel) : PlayPayment := {
  spelling := "using its <label> ability",
  body :=
    Semantics.Primitives.PlayPayment.byKeyword label
}
