/-- "copy <subject> [<times> times] [, except …]": a copy of a spell or of an ability on the stack,
put onto the stack (Twincast's spell, Strionic Resonator's triggered ability, Fork's red copy)
[CR#707.10]. -/
-- [CR#707.10]
template copyOnStack (subject : NounPhrase) (times : Amount := 1)
    (exceptions : List CopyExcept := []) : Action := {
  -- TODO(spelling): Format and coordinate the exceptions list entries, including the empty case.
  body := copy .fromStack subject times exceptions
}
