/-- "there is an additional <part> [after <anchor>] [, followed by an additional <next>]": phases
added to the current turn [CR#500.8]. -/
-- [CR#500.8]
primitive additionalPhase (part : TurnPart) (anchor : Option TurnPart) (count : Amount)
    (followedBy : Option TurnPart := none) : Action := {
  -- TODO(spelling): Context-dependent wording: there is an additional <part> [after <anchor>] [, followed by an additional <next>]; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Primitives.Action.addTurnPart part anchor count followedBy
}
