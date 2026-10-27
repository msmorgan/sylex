/-- "copy <subject> [<times> times] [except …]", a spell or a card [CR#707.10,707.12]. -/
-- [CR#707.10,707.12]
primitive Action.copy (sort : CopySort) (subject : NounPhrase) (times : Amount)
    (exceptions : List CopyExcept) : Action := {
  -- TODO(spelling): Context-dependent wording: copy <subject> [<times> times] [except …]; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Primitives.Action.copy sort subject times exceptions
}
