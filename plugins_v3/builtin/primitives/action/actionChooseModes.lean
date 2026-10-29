/-- Choose modes from the given modal choices. -/
primitive actionChooseModes (quantity : Quantity) (modes : List Mode) : Action := {
  body := Semantics.Primitives.Action.chooseModes quantity modes
}
