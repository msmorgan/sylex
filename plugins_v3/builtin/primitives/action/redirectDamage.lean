/-- "<cut> [of <damage>] is dealt to <to> instead": a redirection effect [CR#614.9]. Under
`cant`, "<damage> can't be dealt instead to <to>". -/
-- [CR#614.9]
primitive redirectDamage (cut : PreventCut) (to : NounPhrase)
    (damage : Option WouldClause := none) : Action := {
  -- TODO(spelling): Context-dependent wording: <cut> [of <damage>] is dealt to <to> instead; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Primitives.Action.redirectDamage cut to damage
}
