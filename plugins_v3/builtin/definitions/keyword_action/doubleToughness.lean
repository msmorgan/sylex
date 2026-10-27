/-- "double <subject>'s toughness": it gets +0/+X, X its toughness (`Definition`,
[CR#701.10b,701.10c]). -/
action doubleToughness (subject : NounPhrase) (duration : Option Duration := none) :
    Action := {
  -- TODO(spelling): double <subject>'s toughness [<duration>]; string substitution cannot form
  -- possessives or omit an absent duration.
  lexical := { verbs := [{ lemma := "double", frames := [Monotransitive] }] },
  deed := "double",
  body :=
    get subject (.up 0)
        (.up (toughnessOf subject))
        duration
}
