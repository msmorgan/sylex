/-- `cost` as what paying it pays: a cost that is itself one plain payment is that payment
[CR#118.1]. -/
-- [CR#118.1]
primitive asPaid (cost : Cost) : Cost := {
  body := Semantics.Primitives.Cost.asPaid cost
}
