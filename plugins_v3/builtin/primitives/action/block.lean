/-- "<subject> blocks [<attacker>]" [CR#509.1a] -/
-- [CR#509.1a]
primitive block (attacker : NounPhrase := Templates.NounPhrase.any) : Action := {
  -- TODO(spelling): Context-dependent wording: <subject> blocks [<attacker>]; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Primitives.Action.block attacker
}
