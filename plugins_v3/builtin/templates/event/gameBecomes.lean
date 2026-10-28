/-- "whenever it becomes <designation>": the game gaining it [CR#731.1,731.1a]. -/
-- [CR#731.1,731.1a]
template gameBecomes (designation : DesignationLabel) : GameEvent := {
  spelling := "whenever it becomes <designation>",
  body :=
    does any (setGameDesignation designation)
}
