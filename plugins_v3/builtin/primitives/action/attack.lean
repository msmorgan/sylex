/-- "<subject> attacks [<defender>]" [CR#508.1a] -/
-- [CR#508.1a]
primitive attack (defender : Option NounPhrase := none) : Action := {
  -- TODO(spelling): Context-dependent wording: <subject> attacks [<defender>]; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Primitives.Action.attack defender
}
