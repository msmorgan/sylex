/-- "transformed by paying <cost> rather than its mana cost" [CR#712.8c,702.146a]. -/
-- [CR#712.8c,702.146a]
primitive transformedPaying (cost : Cost) : PlayPayment := {
  spelling := "transformed by paying <cost> rather than its mana cost",
  body :=
    Semantics.Primitives.PlayPayment.transformed cost
}
