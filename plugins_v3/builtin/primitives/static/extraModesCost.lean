/-- For each mode chosen beyond the first, pay an additional cost, as with escalate.
[CR#702.120a] -/
primitive extraModesCost (cost : Cost) : ModeChange := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.ModeChange.extraModesCost cost
}
