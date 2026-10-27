/-- "triple <subject>'s power": it gets +X/+0, X twice its power (`Definition`,
[CR#701.11b,701.11c]). -/
action triplePower (subject : NounPhrase) (duration : Option Duration := none) :
    Action := {
  -- TODO(spelling): triple <subject>'s power [<duration>]; string substitution cannot form
  -- possessives or omit an absent duration.
  lexical := { verbs := [{ lemma := "triple", frames := [Monotransitive] }] },
  deed := "triple",
  body :=
    get subject
        (.up (times 2 (powerOf subject)))
        (.up 0) duration
}
