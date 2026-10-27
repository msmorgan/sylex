/-- "a [<state>] door of <room>" [CR#709.5j] -/
-- [CR#709.5j]
primitive doorOf (state : Option LockState) (room : NounPhrase) : Door := {
  -- TODO(spelling): Context-dependent wording: a [<state>] door of <room>; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Primitives.Door.doorOf state room
}
