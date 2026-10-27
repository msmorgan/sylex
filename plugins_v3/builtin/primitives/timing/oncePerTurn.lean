/-- "only once each turn" [CR#602.5b] -/
-- [CR#602.5b]
primitive oncePerTurn : UsageLimit := {
  spelling := "only once each turn",
  body :=
    Semantics.Primitives.UsageLimit.mk 1 Semantics.Period.turn
}
