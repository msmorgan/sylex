/-- "a [<state>] door of <room>" [CR#709.5j] -/
-- [CR#709.5j]
primitive doorOf (state : Option LockState) (room : NounPhrase) : Door := {
  -- TODO(spelling): Include or omit the clauses supplied by state.
  body :=
    Semantics.Primitives.Door.doorOf state room
}
