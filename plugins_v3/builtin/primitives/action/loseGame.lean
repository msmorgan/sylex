/-- "<subject> loses the game [for <cause>]": the cause rides the act ("can't lose the game for
having 0 or less life" [CR#704.5a]). -/
-- [CR#704.5a]
primitive loseGame (cause : Option StateBasedCause := none) : Action := {
  -- TODO(spelling): Context-dependent wording: <subject> loses the game [for <cause>]; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Primitives.Action.conclude Semantics.OutcomeVerb.loseGame cause
}
