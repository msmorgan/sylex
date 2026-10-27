/-- "As <subject> enters, [secretly] choose <chosen>": a replacement effect whose choice is made
before the permanent enters [CR#603.6d,614.1c,614.12a]. -/
-- [CR#603.6d,614.1c,614.12a]
template entryChoice (subject chosen : NounPhrase) (disclosure : Disclosure := .openly) :
    StaticSpec := {
  -- TODO(spelling): Context-dependent wording: As <subject> enters, [secretly] choose <chosen>; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Templates.StaticSpec.replace
      (Semantics.Templates.WouldClause.would (Semantics.Templates.Subject.actor subject) Semantics.Templates.Action.enter)
      (Semantics.Templates.Replacement.before (Semantics.Templates.Action.actionChoose Option.none chosen disclosure Option.none) Option.none)
      Option.none Semantics.ReplUse.repeatedly Option.none
}
