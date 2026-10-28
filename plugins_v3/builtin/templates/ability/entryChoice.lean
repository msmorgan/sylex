/-- "As <subject> enters, [secretly] choose <chosen>": a replacement effect whose choice is made
before the permanent enters [CR#603.6d,614.1c,614.12a]. -/
-- [CR#603.6d,614.1c,614.12a]
template entryChoice (subject chosen : NounPhrase) (disclosure : Disclosure := .openly) :
    StaticSpec := {
  -- TODO(spelling): Context-dependent wording: As <subject> enters, [secretly] choose <chosen>; string substitution cannot express its omitted or inflected parts.
  body :=
    replace
      (would (actor subject) enter)
      (before (actionChoose none chosen disclosure none) none)
      none Semantics.ReplUse.repeatedly none
}
