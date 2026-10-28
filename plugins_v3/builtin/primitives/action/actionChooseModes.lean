/-- Choose modes from the given modal choices. -/
primitive actionChooseModes (quantity : Quantity) (modes : List (Option Cost × Option Nat × Action)) : Action := {
  body := Semantics.Primitives.Action.chooseModes quantity modes
}
