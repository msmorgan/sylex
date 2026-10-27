primitive actionChooseModes (quantity : Semantics.Quantity) (modes : List (Option Semantics.Cost × Option Nat × Semantics.Action)) : Semantics.Action := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.Action.chooseModes quantity modes
}
