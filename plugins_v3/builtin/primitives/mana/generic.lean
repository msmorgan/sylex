/-- A Mana Substitute's act pays one generic mana [CR#702.51a,702.66a,702.126a,701.67a]. -/
primitive generic : PipPaid := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.PipPaid.generic
}
