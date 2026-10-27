primitive allInstead (cost : Option Cost := none) (condition : Option Condition := none) :
    ModeChange := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.ModeChange.allInstead cost condition
}
