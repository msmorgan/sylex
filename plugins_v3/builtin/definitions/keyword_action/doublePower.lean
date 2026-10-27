/-- "double <subject>'s power" (`Definition`, [CR#701.10b,701.10c]): it gets +X/+0, X its power. -/
action doublePower (subject : NounPhrase) (duration : Option Duration := none) :
    Action := {
  -- TODO(spelling): double <subject>'s power [<duration>]; string substitution cannot form
  -- possessives or omit an absent duration.
  lexical := { verbs := [{ lemma := "double", frames := [Monotransitive] }] },
  deed := "double",
  body :=
    get subject
        (.up (powerOf subject)) (.up 0)
        duration
}
