/-- "copy <subject> [<times> times] [, except …]": a copy of a spell or of an ability on the stack,
put onto the stack (Twincast's spell, Strionic Resonator's triggered ability, Fork's red copy)
[CR#707.10]. -/
-- [CR#707.10]
primitive copyOnStack (subject : NounPhrase) (times : Amount := 1)
    (exceptions : List CopyExcept := []) : Action := {
  -- TODO(spelling): Context-dependent wording: copy <subject> [<times> times] [, except …]; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Primitives.Action.copy Semantics.CopySort.fromStack subject times exceptions
}
