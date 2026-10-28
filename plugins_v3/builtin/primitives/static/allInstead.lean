/-- Choose all modes instead, for a cost (entwine [CR#702.42a]) or under a condition. -/
primitive allInstead (cost : Option Cost := none) (condition : Option Condition := none) :
    ModeChange := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.ModeChange.allInstead cost condition
}
