/-- An Action performed whenever the condition holds, without using the stack, as with ascend on
a permanent. [CR#113.9,702.131b] -/
primitive whileHolds (condition : Condition) (does : Action) : StaticSpec := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.StaticSpec.whileHolds condition does
}
