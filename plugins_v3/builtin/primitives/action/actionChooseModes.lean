/-- Choose modes from the given modal choices. -/
primitive actionChooseModes (quantity : Quantity) (modes : List (Option Cost × Option Nat × Action)) : Action := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.Action.chooseModes quantity modes
}
