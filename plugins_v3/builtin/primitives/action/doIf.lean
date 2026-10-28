/-- "if <condition>, <action>[; otherwise, <otherwise>]" -/
primitive doIf (condition : Condition) (action : Action) (otherwise : Option Action := none) (postposed : Bool := false) : Action := {
  body := Semantics.Primitives.Action.doIf condition action otherwise postposed
}
