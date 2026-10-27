/-- "converted by paying <cost> rather than its mana cost": converting is transforming written
with another verb [CR#701.28a], and a converted spell is cast as a transformed one is
[CR#712.8c,702.162a]. -/
-- [CR#701.28a]
-- [CR#712.8c,702.162a]
primitive convertedPaying (cost : Cost) : PlayPayment := {
  spelling := "converted by paying <cost> rather than its mana cost",
  body :=
    Semantics.Primitives.PlayPayment.transformed cost
}
