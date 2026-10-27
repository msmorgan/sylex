/-- "it becomes <designation>": the game gains a designation the game can have [CR#731.1]. -/
-- [CR#731.1]
primitive setGameDesignation (designation : DesignationLabel) : Action := {
  spelling := "it becomes <designation>",
  body :=
    Semantics.Primitives.Action.gameGains designation
}
