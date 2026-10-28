/-- Sets how many modes are chosen: spree chooses one or more, while tiered chooses one.
[CR#702.172a,702.183a] -/
primitive setQuantity (quantity : Quantity) : ModeChange := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.ModeChange.setQuantity quantity
}
