/-- "copy <subject> [<times> times] [except …]", a spell or a card [CR#707.10,707.12]. -/
-- [CR#707.10,707.12]
primitive copy (sort : CopySort) (subject : NounPhrase) (times : Amount)
    (exceptions : List CopyExcept) : Action := {
  -- TODO(spelling): Format and coordinate the exceptions list entries, including the empty case.
  body :=
    Semantics.Primitives.Action.copy sort subject times exceptions
}
